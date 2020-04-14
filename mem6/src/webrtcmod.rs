// webrtcmod.rs
//! module that cares about WebRTC communication

#![allow(clippy::panic)]

// region: use
use crate::*;

//use mem6_common::*;

use unwrap::unwrap;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use js_sys::Reflect;
//use web_sys::{ErrorEvent};
use serde_derive::{Serialize, Deserialize};
// endregion

#[derive(Serialize, Deserialize, Clone)]
struct MyCandidate {
    candidate: String,
    sdp_m_line_index: Option<u16>,
    sdp_mid: Option<String>,
}

/// I must send vdom, because rrc cannot be used inside async block
pub fn web_rtc_start(vdom: dodrio::VdomWeak, rrc: &mut RootRenderingComponent) {
    websysmod::debug_write("web_rtc_start()");
    let receiver_ws_uid = websysmod::get_input_element_value_string_by_id("receiver_ws_uid");
    let receiver_ws_uid = unwrap!(receiver_ws_uid.parse::<usize>());
    rrc.web_data.rtc_receiver_ws_uid = receiver_ws_uid;
    //Result<RtcPeerConnection, JsValue>
    let result_pc = web_sys::RtcPeerConnection::new();
    if let Ok(pc) = result_pc {
        //websysmod::debug_write("web_rtc_start ok");
        // move the connection to the struct that is globally available
        rrc.web_data.rtc_peer_connection = Some(pc);
        let mut pc = rrc.web_data.rtc_peer_connection.as_ref().unwrap().clone();
        //set local ice candidate event
        let v2 = vdom.clone();
        set_on_local_icecandidate(v2, &mut pc);
        spawn_local(async move {
            //websysmod::debug_write("web_rtc_start async");

            let mut data_channel = set_on_msg(&pc);
            //on open must be second, because the on_msg creates the datachannel
            set_on_open(&mut data_channel);

            //promise to future and await returns RtcIdentityAssertion
            let o = wasm_bindgen_futures::JsFuture::from(pc.create_offer()).await;
            let o = unwrap!(o);
            let o = unwrap!(o.dyn_into::<web_sys::RtcSessionDescriptionInit>());
            //websysmod::debug_write(&format!("RtcSessionDescriptionInit: {:?}", &o));

            let r = wasm_bindgen_futures::JsFuture::from(pc.set_local_description(&o)).await;
            //websysmod::debug_write(&format!("result set_local_description: {:?}", &r));
            // Option<RtcSessionDescription>
            let x = unwrap!(pc.local_description());

            //websysmod::debug_write(&format!("local_description: {:?}", &x.sdp()));
            unwrap!(
                vdom.with_component({
                    move |root| {
                        let rrc = root.unwrap_mut::<RootRenderingComponent>();
                        rrc.web_data.rtc_data_channel = Some(data_channel);
                        web_rtc_send_offer(rrc, rrc.web_data.rtc_receiver_ws_uid, x.sdp());
                    }
                })
                .await
            );
        });
    }
}

/// send offer over websocket to establish peer connection
pub fn web_rtc_send_offer(rrc: &RootRenderingComponent, receiver_ws_uid: usize, sdp: String) {
    //websysmod::debug_write("web_rtc_send_offer()");
    let msg_receivers_json = gamedatamod::prepare_json_msg_receivers_for_one(receiver_ws_uid);

    let msg = websocketmod::WsMessageForReceivers {
        msg_sender_ws_uid: rrc.web_data.my_ws_uid,
        msg_receivers_json: msg_receivers_json,
        msg_data: gamedatamod::WsMessageGameData::MsgWebrtcOffer { sdp },
    };
    rrc.web_data.send_ws_msg(&msg);
}

/// receive offer
pub fn web_rtc_receive_offer(
    vdom: dodrio::VdomWeak,
    rrc: &mut RootRenderingComponent,
    sdp: String,
    msg_sender_ws_uid: usize,
) {
    let result_pc = web_sys::RtcPeerConnection::new();
    if let Ok(pc) = result_pc {
        rrc.web_data.rtc_receiver_ws_uid = msg_sender_ws_uid;
        vdom.schedule_render();
        // move the connection to the struct that is globally available
        rrc.web_data.rtc_peer_connection = Some(pc);
        let pc = rrc.web_data.rtc_peer_connection.as_ref().unwrap().clone();
        spawn_local(async move {
            //websysmod::debug_write(&format!("web_rtc_receive_offer: {}", &sdp));
            let mut rd = web_sys::RtcSessionDescriptionInit::new(web_sys::RtcSdpType::Offer);
            //set_sdp with a wrong name
            rd.sdp(&sdp);
            let r = wasm_bindgen_futures::JsFuture::from(pc.set_remote_description(&rd)).await;
            //websysmod::debug_write(&format!("result set_remote_description: {:?}", &r));

            let mut data_channel = set_on_msg(&pc);
            //on open must be second, because the on_msg creates the datachannel
            set_on_open(&mut data_channel);

            let o = wasm_bindgen_futures::JsFuture::from(pc.create_answer()).await;
            let o = unwrap!(o);
            let o = unwrap!(o.dyn_into::<web_sys::RtcSessionDescriptionInit>());
            let r = wasm_bindgen_futures::JsFuture::from(pc.set_local_description(&o)).await;
            //websysmod::debug_write(&format!("result set_local_description: {:?}", &r));

            // Option<RtcSessionDescription>
            let x = unwrap!(pc.local_description());
            //websysmod::debug_write(&format!("local_description: {:?}", &x.sdp()));

            unwrap!(
                vdom.with_component({
                    move |root| {
                        let rrc = root.unwrap_mut::<RootRenderingComponent>();
                        web_rtc_send_answer(rrc, rrc.web_data.rtc_receiver_ws_uid, x.sdp());
                        rrc.web_data.rtc_data_channel = Some(data_channel);
                    }
                })
                .await
            );
        });
    }
}

/// set callbacks for on receive webrtc message
pub fn set_on_msg(pc: &web_sys::RtcPeerConnection) -> web_sys::RtcDataChannel {
    //websysmod::debug_write(&format!("set_on_msg: {}", ""));
    let mut dic = web_sys::RtcDataChannelInit::new();
    dic.negotiated(true);
    dic.id(0);
    let dc = pc.create_data_channel_with_data_channel_dict("BackChannel", &dic);
    //websysmod::debug_write(&format!("create_data_channel: {:?}", &dc));
    let msg_recv_handler = Box::new(move |msg: JsValue| {
        let data: JsValue = unwrap!(Reflect::get(&msg, &"data".into()));
        let data = unwrap!(data.as_string());
        web_rtc_receive_chat(data);
    });
    let cb_oh: Closure<dyn Fn(JsValue)> = Closure::wrap(msg_recv_handler);
    dc.set_onmessage(Some(cb_oh.as_ref().unchecked_ref()));
    //websysmod::debug_write(&format!("set_onmessage: {}", ""));
    cb_oh.forget();
    //return
    dc
}

/// on receive webrtc message
pub fn set_on_open(dc: &mut web_sys::RtcDataChannel) {
    let msg_recv_handler = Box::new(move |msg: JsValue| {
        websysmod::debug_write("on open handle");
    });
    let cb_oh: Closure<dyn Fn(JsValue)> = Closure::wrap(msg_recv_handler);
    dc.set_onopen(Some(cb_oh.as_ref().unchecked_ref()));
    //websysmod::debug_write(&format!("set_onopen: {}", ""));
    cb_oh.forget();
}

/// send answer over websocket to establish peer connection
pub fn web_rtc_send_answer(rrc: &RootRenderingComponent, receiver_ws_uid: usize, sdp: String) {
    //websysmod::debug_write("web_rtc_send_answer()");
    let msg_receivers_json = gamedatamod::prepare_json_msg_receivers_for_one(receiver_ws_uid);

    let msg = websocketmod::WsMessageForReceivers {
        msg_sender_ws_uid: rrc.web_data.my_ws_uid,
        msg_receivers_json: msg_receivers_json,
        msg_data: gamedatamod::WsMessageGameData::MsgWebrtcAnswer { sdp: sdp },
    };
    rrc.web_data.send_ws_msg(&msg);
}

/// receive answer
pub fn web_rtc_receive_answer(
    vdom: dodrio::VdomWeak,
    rrc: &mut RootRenderingComponent,
    sdp: String,
) {
    let mut pc = rrc.web_data.rtc_peer_connection.as_ref().unwrap().clone();
    spawn_local(async move {
        //websysmod::debug_write(&format!("web_rtc_receive_answer: {}", &sdp));

        let mut rd = web_sys::RtcSessionDescriptionInit::new(web_sys::RtcSdpType::Answer);
        //set_sdp with a wrong name
        rd.sdp(&sdp);
        let r = wasm_bindgen_futures::JsFuture::from(pc.set_remote_description(&rd)).await;
        //websysmod::debug_write(&format!("result set_remote_description: {:?}", &r));
        unwrap!(
            vdom.with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    rrc.web_data.rtc_accepted_call = true;
                    web_rtc_send_ice_candidates(rrc, rrc.web_data.rtc_receiver_ws_uid);
                }
            })
            .await
        );
    });
}

/// this is candidate for the local object (not remote)
pub fn set_on_local_icecandidate(vdom: dodrio::VdomWeak, pc: &mut web_sys::RtcPeerConnection) {
    let handler = Box::new(move |event: JsValue| {
        let v2 = vdom.clone();
        //websysmod::debug_write("on local icecandidate");
        //send the candidate to the remote peer over ws
        let event = unwrap!(event.dyn_into::<web_sys::RtcPeerConnectionIceEvent>());
        if let Some(candidate) = event.candidate() {
            //websysmod::debug_write(&format!("candidate: {:?}", &candidate));
            //workaround because to_json dows not work well
            let candidate_json = MyCandidate {
                candidate: candidate.candidate(),
                sdp_m_line_index: candidate.sdp_m_line_index(),
                sdp_mid: candidate.sdp_mid(),
            };
            let candidate_json = unwrap!(serde_json::to_string(&candidate_json));

            //websysmod::debug_write(&format!("candidate_json: {:?}", &candidate_json));
            spawn_local(async move {
                unwrap!(
                    v2.with_component({
                        move |root| {
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();
                            //save to queue
                            rrc.web_data.rtc_ice_queue.push(candidate_json);
                            if rrc.web_data.rtc_accepted_call == true {
                                web_rtc_send_ice_candidates(rrc, rrc.web_data.rtc_receiver_ws_uid);
                            }
                        }
                    })
                    .await
                );
            });
        }
    });
    let cb_oh: Closure<dyn Fn(JsValue)> = Closure::wrap(handler);
    pc.set_onicecandidate(Some(cb_oh.as_ref().unchecked_ref()));
    //websysmod::debug_write(&format!("set_on_local_icecandidate: {}", ""));
    cb_oh.forget();
}

/// send offer over websocket to establish peer connection
pub fn web_rtc_send_ice_candidates(rrc: &mut RootRenderingComponent, receiver_ws_uid: usize) {
    for sdp in &rrc.web_data.rtc_ice_queue {
        //websysmod::debug_write("web_rtc_send_icecandidate()");
        let msg_receivers_json = gamedatamod::prepare_json_msg_receivers_for_one(receiver_ws_uid);
        let sdp = sdp.to_string();
        let msg = websocketmod::WsMessageForReceivers {
            msg_sender_ws_uid: rrc.web_data.my_ws_uid,
            msg_receivers_json: msg_receivers_json,
            msg_data: gamedatamod::WsMessageGameData::MsgWebrtcIceCandidate { sdp },
        };
        rrc.web_data.send_ws_msg(&msg);
    }
    rrc.web_data.rtc_ice_queue.truncate(0);
}

/// receive ice candidate
pub fn web_rtc_receive_ice_candidate(
    vdom: dodrio::VdomWeak,
    rrc: &mut RootRenderingComponent,
    sdp: String,
) {
    let mut pc = rrc.web_data.rtc_peer_connection.as_ref().unwrap().clone();
    spawn_local(async move {
        //websysmod::debug_write(&format!("web_rtc_receive_ice_candidate{}", ""));
        let my_candidate: MyCandidate = unwrap!(serde_json::from_str(&sdp));
        let mut icecandidate_init = web_sys::RtcIceCandidateInit::new("");
        icecandidate_init.candidate(&my_candidate.candidate);
        icecandidate_init.sdp_mid(Some(unwrap!(my_candidate.sdp_mid).as_str()));
        icecandidate_init.sdp_m_line_index(my_candidate.sdp_m_line_index);
        //websysmod::debug_write(&format!("icecandidate_init: {:?}", &icecandidate_init));
        let icecandidate = web_sys::RtcIceCandidate::new(&icecandidate_init);
        //websysmod::debug_write(&format!("icecandidate: {:?}", &icecandidate));
        let icecandidate = unwrap!(icecandidate);
        // if the WebRtcPeerConnection is not in the right state, then queue the icecandidate in a vector

        if pc.remote_description().is_some() {
            //websysmod::debug_write(&format!("remote type: {:?}",&pc.remote_description().unwrap().type_()));
            //websysmod::debug_write(&format!("add_ice_candidate: {}", ""));
            let r = wasm_bindgen_futures::JsFuture::from(
                pc.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&icecandidate)),
            )
            .await;
        //websysmod::debug_write(&format!("result add_ice_candidate: {:?}", &r));
        } else {
            //websysmod::debug_write(&format!("push to queue: {:?}", ""));
            //push to a queue
        }
    });
}

/// send over webrtc
pub fn web_rtc_send_chat(rrc: &RootRenderingComponent, msg: String) {
    //websysmod::debug_write("web_rtc_send_chat()");
    let dc = unwrap!(rrc.web_data.rtc_data_channel.as_ref());
    match dc.ready_state() {
        web_sys::RtcDataChannelState::Connecting => {
            websysmod::debug_write("Connection not open; queueing: ");
        }
        web_sys::RtcDataChannelState::Open => {
            unwrap!(dc.send_with_str(&msg));
        }
        web_sys::RtcDataChannelState::Closing => {
            websysmod::debug_write("Attempted to send message while closing: ");
        }
        web_sys::RtcDataChannelState::Closed => {
            websysmod::debug_write("Error! Attempt to send while connection closed.");
        }
        _ => {
            websysmod::debug_write("Some other ready_state ??.");
        }
    }
}

/// receive msg
pub fn web_rtc_receive_chat(text: String) {
    websysmod::debug_write(&format!("web_rtc_receive: {}", &text));
    websysmod::set_input_element_value_string_by_id("received_message", &text);
}
