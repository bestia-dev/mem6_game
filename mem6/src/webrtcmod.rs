// webrtcmod.rs
//! module for WebRTC communication

#![allow(clippy::panic)]

// region: use
use crate::*;
use crate::htmltemplatemod::HtmlTemplating;

//use mem6_common::*;

use unwrap::unwrap;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use js_sys::Reflect;
use dodrio::{RenderContext, Node, VdomWeak};
use serde_derive::{Serialize, Deserialize};
// endregion

/// one chat message looks like this
pub struct ChatMessage {
    pub time: usize,
    pub sender: usize,
    pub msg: String,
}
/// game data
pub struct WebRtcData {
    /// webrtc connection
    pub rtc_peer_connection: Option<web_sys::RtcPeerConnection>,
    /// rtc data channel
    pub rtc_data_channel: Option<web_sys::RtcDataChannel>,
    /// receiver for webrtc
    pub rtc_receiver_ws_uid: usize,
    /// accepted call
    pub rtc_accepted_call: bool,
    /// queue for ice candidate
    pub rtc_ice_queue: Vec<String>,
    /// chat messages
    pub rtc_chat: Vec<ChatMessage>,
    /// is_rtc_data_channel_open
    pub is_rtc_data_channel_open: bool,
    /// if a render event comes while we are typing in input
    /// we will loose the content. So on every keyup, I have to store it
    /// in a field in rrc
    pub rtc_my_message: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct IceCandidate {
    candidate: String,
    sdp_m_line_index: Option<u16>,
    sdp_mid: Option<String>,
}

impl WebRtcData {
    /// constructor
    pub fn new() -> Self {
        // return from constructor
        WebRtcData {
            rtc_peer_connection: None,
            rtc_data_channel: None,
            rtc_accepted_call: false,
            rtc_ice_queue: vec![],
            rtc_receiver_ws_uid: 0,
            rtc_chat: vec![],
            is_rtc_data_channel_open: false,
            rtc_my_message: "".to_string(),
        }
    }
}

/// I must send vdom, because rrc cannot be used inside async block
/// if it is not static. But for me it is static, but how to say to rust?
pub fn web_rtc_start(vdom: VdomWeak, wrtc: &mut WebRtcData) {
    websysmod::debug_write("web_rtc_start()");
    let receiver_ws_uid = websysmod::get_input_element_value_string_by_id("receiver_ws_uid");
    let receiver_ws_uid = unwrap!(receiver_ws_uid.parse::<usize>());
    wrtc.rtc_receiver_ws_uid = receiver_ws_uid;
    let conf_str = r#"[{ "url": "stun:stun.bestia.dev" }] "#;
    let res_jsvalue_jsvalue = js_sys::JSON::parse(conf_str);
    websysmod::debug_write(&format!("res_jsvalue_jsvalue {:?}", res_jsvalue_jsvalue));
    let js_value = unwrap!(res_jsvalue_jsvalue);
    //ice_servers(&mut self, val: &JsValue) -> &mut Self
    let mut rtc_conf = web_sys::RtcConfiguration::new();
    let rtc_conf = rtc_conf.ice_servers(&js_value);
    //Result<RtcPeerConnection, JsValue>
    let result_pc =
        web_sys::RtcPeerConnection::new_with_configuration_and_constraints(&rtc_conf, None);
    websysmod::debug_write(&format!("result_pc {:?}", result_pc));
    if let Ok(pc) = result_pc {
        //websysmod::debug_write("web_rtc_start ok");
        // move the connection to the struct that is globally available
        wrtc.rtc_peer_connection = Some(pc);
        let mut pc = wrtc.rtc_peer_connection.as_ref().unwrap().clone();
        //set local ice candidate event
        web_rtc_set_on_local_ice_candidate(vdom.clone(), &mut pc);
        let mut data_channel = web_rtc_set_on_msg(vdom.clone(), &pc);
        //on open must be second, because the on_msg creates the datachannel
        web_rtc_set_on_open(vdom.clone(), &mut data_channel);

        spawn_local({
            let vdom_on_next_tick = vdom.clone();
            async move {
                //websysmod::debug_write("web_rtc_start async");
                //promise to future and await returns RtcIdentityAssertion
                let offer_init = wasm_bindgen_futures::JsFuture::from(pc.create_offer()).await;
                let offer_init = unwrap!(offer_init);
                let offer_init =
                    unwrap!(offer_init.dyn_into::<web_sys::RtcSessionDescriptionInit>());
                //websysmod::debug_write(&format!("RtcSessionDescriptionInit: {:?}", &o));

                let _result =
                    wasm_bindgen_futures::JsFuture::from(pc.set_local_description(&offer_init))
                        .await;
                //websysmod::debug_write(&format!("result set_local_description: {:?}", &r));
                // Option<RtcSessionDescription>
                let local_description = unwrap!(pc.local_description());

                //websysmod::debug_write(&format!("local_description: {:?}", &local_description.sdp()));
                unwrap!(
                    vdom_on_next_tick
                        .with_component({
                            move |root| {
                                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                                rrc.web_rtc_data.rtc_data_channel = Some(data_channel);
                                web_rtc_send_offer(
                                    rrc,
                                    rrc.web_rtc_data.rtc_receiver_ws_uid,
                                    local_description.sdp(),
                                );
                            }
                        })
                        .await
                );
            }
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
    vdom: VdomWeak,
    rrc: &mut RootRenderingComponent,
    sdp: String,
    msg_sender_ws_uid: usize,
) {
    websysmod::debug_write("web_rtc_receive_offer()");
    let result_pc = web_sys::RtcPeerConnection::new();
    if let Ok(pc) = result_pc {
        rrc.web_rtc_data.rtc_receiver_ws_uid = msg_sender_ws_uid;
        vdom.schedule_render();
        // move the connection to the struct that is globally available
        rrc.web_rtc_data.rtc_peer_connection = Some(pc);
        let pc = rrc
            .web_rtc_data
            .rtc_peer_connection
            .as_ref()
            .unwrap()
            .clone();
        //websysmod::debug_write(&format!("web_rtc_receive_offer: {}", &sdp));
        let mut init_offer = web_sys::RtcSessionDescriptionInit::new(web_sys::RtcSdpType::Offer);
        //set_sdp with a wrong name
        init_offer.sdp(&sdp);
        spawn_local({
            let vdom_on_next_tick = vdom.clone();
            async move {
                let _result =
                    wasm_bindgen_futures::JsFuture::from(pc.set_remote_description(&init_offer))
                        .await;
                //websysmod::debug_write(&format!("result set_remote_description: {:?}", &r));

                let mut data_channel = web_rtc_set_on_msg(vdom.clone(), &pc);
                //on open must be second, because the on_msg creates the datachannel
                web_rtc_set_on_open(vdom.clone(), &mut data_channel);

                let answer_init = wasm_bindgen_futures::JsFuture::from(pc.create_answer()).await;
                let answer_init = unwrap!(answer_init);
                let answer_init =
                    unwrap!(answer_init.dyn_into::<web_sys::RtcSessionDescriptionInit>());
                let _result =
                    wasm_bindgen_futures::JsFuture::from(pc.set_local_description(&answer_init))
                        .await;
                //websysmod::debug_write(&format!("result set_local_description: {:?}", &r));

                // Option<RtcSessionDescription>
                let local_description = unwrap!(pc.local_description());
                //websysmod::debug_write(&format!("local_description: {:?}", &local_description.sdp()));

                unwrap!(
                    vdom_on_next_tick
                        .with_component({
                            move |root| {
                                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                                web_rtc_send_answer(
                                    rrc,
                                    rrc.web_rtc_data.rtc_receiver_ws_uid,
                                    local_description.sdp(),
                                );
                                rrc.web_rtc_data.rtc_data_channel = Some(data_channel);
                            }
                        })
                        .await
                );
            }
        });
    }
}

/// set callbacks for on receive webrtc message
pub fn web_rtc_set_on_msg(
    vdom: VdomWeak,
    pc: &web_sys::RtcPeerConnection,
) -> web_sys::RtcDataChannel {
    //websysmod::debug_write(&format!("web_rtc_set_on_msg: {}", ""));
    let mut dic = web_sys::RtcDataChannelInit::new();
    dic.negotiated(true);
    dic.id(0);
    let dc = pc.create_data_channel_with_data_channel_dict("BackChannel", &dic);
    //websysmod::debug_write(&format!("create_data_channel: {:?}", &dc));
    let msg_recv_handler = Box::new(move |msg: JsValue| {
        let data: JsValue = unwrap!(Reflect::get(&msg, &"data".into()));
        let data = unwrap!(data.as_string());
        web_rtc_receive_chat(vdom.clone(), data);
    });
    let cb_oh: Closure<dyn Fn(JsValue)> = Closure::wrap(msg_recv_handler);
    dc.set_onmessage(Some(cb_oh.as_ref().unchecked_ref()));
    //websysmod::debug_write(&format!("set_onmessage: {}", ""));
    cb_oh.forget();
    //return
    dc
}

/// on receive webrtc message
pub fn web_rtc_set_on_open(vdom: VdomWeak, dc: &mut web_sys::RtcDataChannel) {
    let msg_recv_handler = Box::new(move |_msg: JsValue| {
        websysmod::debug_write("web_rtc on_open");
        spawn_local({
            let vdom_on_next_tick = vdom.clone();
            async move {
                unwrap!(
                    vdom_on_next_tick
                        .with_component({
                            let vdom = vdom_on_next_tick.clone();
                            move |root| {
                                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                                rrc.web_rtc_data.is_rtc_data_channel_open = true;
                                vdom.schedule_render();
                            }
                        })
                        .await
                );
            }
        });
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
pub fn web_rtc_receive_answer(vdom: VdomWeak, rrc: &mut RootRenderingComponent, sdp: String) {
    websysmod::debug_write("web_rtc_receive_answer");
    let pc = rrc
        .web_rtc_data
        .rtc_peer_connection
        .as_ref()
        .unwrap()
        .clone();
    spawn_local({
        async move {
            let vdom_on_next_tick = vdom.clone();
            let mut init_answer =
                web_sys::RtcSessionDescriptionInit::new(web_sys::RtcSdpType::Answer);
            //set_sdp with a wrong name
            init_answer.sdp(&sdp);
            let _result =
                wasm_bindgen_futures::JsFuture::from(pc.set_remote_description(&init_answer)).await;
            //websysmod::debug_write(&format!("result set_remote_description: {:?}", &r));
            unwrap!(
                vdom_on_next_tick
                    .with_component({
                        move |root| {
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();
                            rrc.web_rtc_data.rtc_accepted_call = true;
                            web_rtc_send_ice_candidates(rrc, rrc.web_rtc_data.rtc_receiver_ws_uid);
                        }
                    })
                    .await
            );
        }
    });
}

/// this is candidate for the local object (not remote)
pub fn web_rtc_set_on_local_ice_candidate(vdom: VdomWeak, pc: &mut web_sys::RtcPeerConnection) {
    let handler = Box::new(move |event: JsValue| {
        let vdom_on_next_tick = vdom.clone();
        websysmod::debug_write("on local ice_candidate");
        //send the candidate to the remote peer over ws
        let event = unwrap!(event.dyn_into::<web_sys::RtcPeerConnectionIceEvent>());
        if let Some(candidate) = event.candidate() {
            //websysmod::debug_write(&format!("candidate: {:?}", &candidate));
            //workaround because to_json dows not work well
            let candidate_json = IceCandidate {
                candidate: candidate.candidate(),
                sdp_m_line_index: candidate.sdp_m_line_index(),
                sdp_mid: candidate.sdp_mid(),
            };
            let candidate_json = unwrap!(serde_json::to_string(&candidate_json));

            //websysmod::debug_write(&format!("candidate_json: {:?}", &candidate_json));
            spawn_local(async move {
                unwrap!(
                    vdom_on_next_tick
                        .with_component({
                            move |root| {
                                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                                //save to queue
                                rrc.web_rtc_data.rtc_ice_queue.push(candidate_json);
                                if rrc.web_rtc_data.rtc_accepted_call == true {
                                    web_rtc_send_ice_candidates(
                                        rrc,
                                        rrc.web_rtc_data.rtc_receiver_ws_uid,
                                    );
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
    //websysmod::debug_write(&format!("web_rtc_set_on_local_ice_candidate: {}", ""));
    cb_oh.forget();
}

/// send offer over websocket to establish peer connection
pub fn web_rtc_send_ice_candidates(rrc: &mut RootRenderingComponent, receiver_ws_uid: usize) {
    for sdp in &rrc.web_rtc_data.rtc_ice_queue {
        //websysmod::debug_write("web_rtc_send_ice_candidate()");
        let msg_receivers_json = gamedatamod::prepare_json_msg_receivers_for_one(receiver_ws_uid);
        let sdp = sdp.to_string();
        let msg = websocketmod::WsMessageForReceivers {
            msg_sender_ws_uid: rrc.web_data.my_ws_uid,
            msg_receivers_json: msg_receivers_json,
            msg_data: gamedatamod::WsMessageGameData::MsgWebrtcIceCandidate { sdp },
        };
        rrc.web_data.send_ws_msg(&msg);
    }
    rrc.web_rtc_data.rtc_ice_queue.truncate(0);
}

/// receive ice candidate
pub fn web_rtc_receive_ice_candidate(
    _vdom: VdomWeak,
    rrc: &mut RootRenderingComponent,
    sdp: String,
) {
    let pc = rrc
        .web_rtc_data
        .rtc_peer_connection
        .as_ref()
        .unwrap()
        .clone();
    spawn_local(async move {
        websysmod::debug_write("web_rtc_receive_ice_candidate");
        let ice_candidate: IceCandidate = unwrap!(serde_json::from_str(&sdp));
        let mut ice_candidate_init = web_sys::RtcIceCandidateInit::new("");
        ice_candidate_init.candidate(&ice_candidate.candidate);
        ice_candidate_init.sdp_mid(Some(unwrap!(ice_candidate.sdp_mid).as_str()));
        ice_candidate_init.sdp_m_line_index(ice_candidate.sdp_m_line_index);
        //websysmod::debug_write(&format!("ice_candidate_init: {:?}", &ice_candidate_init));
        let ice_candidate = web_sys::RtcIceCandidate::new(&ice_candidate_init);
        //websysmod::debug_write(&format!("ice_candidate: {:?}", &ice_candidate));
        let ice_candidate = unwrap!(ice_candidate);
        // if the WebRtcPeerConnection is not in the right state, then queue the ice_candidate in a vector

        if pc.remote_description().is_some() {
            //websysmod::debug_write(&format!("remote type: {:?}",&pc.remote_description().unwrap().type_()));
            //websysmod::debug_write(&format!("add_ice_candidate: {}", ""));
            let _r = wasm_bindgen_futures::JsFuture::from(
                pc.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&ice_candidate)),
            )
            .await;
        //websysmod::debug_write(&format!("result add_ice_candidate: {:?}", &r));
        } else {
            //for now I didn't need a queue for this. Only for IceCandidate.
        }
    });
}

/// button click or enter key
pub fn web_rtc_send_chat(vdom: VdomWeak, rrc: &mut RootRenderingComponent) {
    let web_rtc_chat_text = websysmod::get_input_element_value_string_by_id("web_rtc_chat_text");
    let dc = unwrap!(rrc.web_rtc_data.rtc_data_channel.as_ref());
    match dc.ready_state() {
        web_sys::RtcDataChannelState::Connecting => {
            websysmod::debug_write("Connection not open; queueing: ");
        }
        web_sys::RtcDataChannelState::Open => {
            unwrap!(dc.send_with_str(&web_rtc_chat_text));
            //save to chat vector
            rrc.web_rtc_data.rtc_chat.push(ChatMessage {
                time: 0,
                sender: 1,
                msg: web_rtc_chat_text,
            })
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
    vdom.schedule_render();
}

/// receive msg
pub fn web_rtc_receive_chat(vdom: VdomWeak, text: String) {
    websysmod::debug_write(&format!("web_rtc_receive: {}", &text));

    //save to chat vector
    spawn_local({
        let vdom_on_next_tick = vdom.clone();
        async move {
            unwrap!(
                vdom_on_next_tick
                    .with_component({
                        let vdom = vdom_on_next_tick.clone();
                        move |root| {
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();
                            rrc.web_rtc_data.rtc_chat.push(ChatMessage {
                                time: 0,
                                sender: 2,
                                msg: text,
                            });
                            vdom.schedule_render();
                        }
                    })
                    .await
            );
        }
    });
}
