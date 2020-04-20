// webrtcmod.rs
//! generic (library) module for WebRTC communication

#![allow(clippy::panic)]

// region: use
use unwrap::unwrap;
use js_sys::Reflect;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use dodrio::{VdomWeak, RootRender};
use serde_derive::{Serialize, Deserialize};
use web_sys::{
    RtcPeerConnection, RtcDataChannel,RtcPeerConnectionIceEvent,RtcConfiguration,
    RtcDataChannelInit,RtcSessionDescriptionInit,RtcSdpType,RtcIceCandidateInit,
    RtcIceCandidate,RtcDataChannelState
};
use web_sys::{WebSocket};

use rust_wasm_websys_utils::*;
// endregion

/// one chat message looks like this
pub struct ChatMessage {
    pub time: usize,
    pub sender: usize,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct IceCandidate {
    candidate: String,
    sdp_m_line_index: Option<u16>,
    sdp_mid: Option<String>,
}

pub trait WebRtcTrait {
    
    // region: getter setter
    fn get_rtc_ws(&self)->&WebSocket;   
    fn set_rtc_ws(&mut self,ws:WebSocket);
    fn get_rtc_my_ws_uid(&self)->usize;
    fn get_rtc_receiver_ws_uid(&self)->usize;
    fn set_rtc_receiver_ws_uid(&mut self, ws_uid:usize);
    fn get_rtc_peer_connection(&self)->RtcPeerConnection;
    fn set_rtc_peer_connection(&mut self,rpc:RtcPeerConnection);
    fn set_rtc_data_channel(&mut self, channel:RtcDataChannel);
    fn set_rtc_is_data_channel_open(&mut self,is_open:bool);
    fn get_rtc_accepted_call(&self)->bool;
    fn set_rtc_accepted_call(&mut self,accepted:bool);
    fn get_mut_rtc_chat(&mut self)->&mut Vec<ChatMessage>;
    fn get_rtc_ice_queue(&self)->&Vec<String>;
    fn get_mut_rtc_ice_queue(&mut self)->&mut Vec<String>;
    fn get_rtc_data_channel(&self)->&RtcDataChannel;
    /// endregion getter setter
    fn get_web_rtc_data_from_root_render(root: &mut dyn RootRender) -> &mut Self;
    fn web_rtc_send_offer(&mut self, sdp: String);
    fn web_rtc_send_answer(&self, sdp: String);
    fn web_rtc_send_ice_candidates(&mut self);
    
    /// I must use vdom.with_components, because rrc cannot be used inside async block
    /// because its lifetime is not static
    fn web_rtc_start(&mut self, vdom: VdomWeak, ws: WebSocket) {
        websysmod::debug_write("web_rtc_start()");
        self.set_rtc_ws(ws);
        self.set_rtc_receiver_ws_uid (
            websysmod::get_input_element_value_usize_by_id("receiver_ws_uid"));
        if let Ok(pc) = Self::web_rtc_new_connection() {
            //websysmod::debug_write("web_rtc_start ok");
            // move the connection to the struct that is globally available
            self.set_rtc_peer_connection(pc);
            let mut pc =  self.get_rtc_peer_connection();
            //set local ice candidate event
            Self::web_rtc_setup_on_local_ice_candidate(vdom.clone(), &mut pc);
            let data_channel = Self::web_rtc_new_data_channel_with_callbacks(vdom.clone(), &pc);

            spawn_local({
                let vdom_on_next_tick = vdom.clone();
                async move {
                    //websysmod::debug_write("web_rtc_start async");
                    //promise to future and await returns RtcIdentityAssertion
                    let offer_init = wasm_bindgen_futures::JsFuture::from(pc.create_offer()).await;
                    let offer_init = unwrap!(offer_init);
                    let offer_init = unwrap!(offer_init.dyn_into::<RtcSessionDescriptionInit>());
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
                                    let wrtc = Self::get_web_rtc_data_from_root_render(root);
                                    wrtc.set_rtc_data_channel(data_channel);
                                    wrtc.web_rtc_send_offer(local_description.sdp());
                                }
                            })
                            .await
                    );
                }
            });
        }
    }

    fn web_rtc_new_connection() -> Result<RtcPeerConnection, JsValue> {
        let conf_str = r#"[{ "url": "stun:stun.bestia.dev" }] "#;
        let res_jsvalue_jsvalue = js_sys::JSON::parse(conf_str);
        websysmod::debug_write(&format!("res_jsvalue_jsvalue {:?}", res_jsvalue_jsvalue));
        let js_value = unwrap!(res_jsvalue_jsvalue);
        //ice_servers(&mut self, val: &JsValue) -> &mut Self
        let mut rtc_conf = RtcConfiguration::new();
        let rtc_conf = rtc_conf.ice_servers(&js_value);
        // return
        RtcPeerConnection::new_with_configuration_and_constraints(&rtc_conf, None)
    }

    /// set callbacks for on receive webrtc message
    fn web_rtc_new_data_channel_with_callbacks(
        vdom: VdomWeak,
        pc: &RtcPeerConnection,
    ) -> RtcDataChannel {
        //websysmod::debug_write(&format!("web_rtc_new_data_channel_with_callbacks: {}", ""));
        let mut dic = RtcDataChannelInit::new();
        dic.negotiated(true);
        dic.id(0);
        let mut data_channel = pc.create_data_channel_with_data_channel_dict("BackChannel", &dic);
        Self::web_rtc_dc_setup_on_msg(vdom.clone(), &mut data_channel);
        Self::web_rtc_dc_setup_on_open(vdom.clone(), &mut data_channel);
        //return
        data_channel
    }

    /// set callbacks for on receive webrtc message
    fn web_rtc_dc_setup_on_msg(vdom: VdomWeak, dc: &mut RtcDataChannel) {
        let event_handler = Box::new(move |msg: JsValue| {
            websysmod::debug_write("web_rtc on_msg");
            let data: JsValue = unwrap!(Reflect::get(&msg, &"data".into()));
            let data = unwrap!(data.as_string());
            Self::web_rtc_receive_chat(vdom.clone(), data);
        });
        let cb_oh: Closure<dyn Fn(JsValue)> = Closure::wrap(event_handler);
        dc.set_onmessage(Some(cb_oh.as_ref().unchecked_ref()));
        cb_oh.forget();
    }

    /// on receive webrtc message
    fn web_rtc_dc_setup_on_open(vdom: VdomWeak, dc: &mut RtcDataChannel) {
        let event_handler = Box::new(move |_msg: JsValue| {
            websysmod::debug_write("web_rtc on_open");
            spawn_local({
                let vdom_on_next_tick = vdom.clone();
                async move {
                    unwrap!(
                        vdom_on_next_tick
                            .with_component({
                                let vdom = vdom_on_next_tick.clone();
                                move |root| {
                                    let wrtc = Self::get_web_rtc_data_from_root_render(root);
                                    wrtc.set_rtc_is_data_channel_open (true);
                                    vdom.schedule_render();
                                }
                            })
                            .await
                    );
                }
            });
        });
        let cb_oh: Closure<dyn Fn(JsValue)> = Closure::wrap(event_handler);
        dc.set_onopen(Some(cb_oh.as_ref().unchecked_ref()));
        //websysmod::debug_write(&format!("set_onopen: {}", ""));
        cb_oh.forget();
    }

    /// receive offer
    fn web_rtc_receive_offer(&mut self, vdom: VdomWeak, sdp: String, msg_sender_ws_uid: usize) {
        websysmod::debug_write("web_rtc_receive_offer()");
        if let Ok(pc) = Self::web_rtc_new_connection() {

            self.set_rtc_receiver_ws_uid (msg_sender_ws_uid);
            vdom.schedule_render();
            // move the connection to the struct that is globally available
            self.set_rtc_peer_connection(pc);
            let pc = self.get_rtc_peer_connection();
            //websysmod::debug_write(&format!("web_rtc_receive_offer: {}", &sdp));
            let mut init_offer = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
            //set_sdp but with a strange name
            init_offer.sdp(&sdp);
            spawn_local({
                let vdom_on_next_tick = vdom.clone();
                async move {
                    let _result = wasm_bindgen_futures::JsFuture::from(
                        pc.set_remote_description(&init_offer),
                    )
                    .await;
                    //websysmod::debug_write(&format!("result set_remote_description: {:?}", &r));

                    let data_channel =
                        Self::web_rtc_new_data_channel_with_callbacks(vdom.clone(), &pc);

                    let answer_init =
                        wasm_bindgen_futures::JsFuture::from(pc.create_answer()).await;
                    let answer_init = unwrap!(answer_init);
                    let answer_init = unwrap!(answer_init.dyn_into::<RtcSessionDescriptionInit>());
                    let _result = wasm_bindgen_futures::JsFuture::from(
                        pc.set_local_description(&answer_init),
                    )
                    .await;
                    //websysmod::debug_write(&format!("result set_local_description: {:?}", &r));

                    // Option<RtcSessionDescription>
                    let local_description = unwrap!(pc.local_description());
                    //websysmod::debug_write(&format!("local_description: {:?}", &local_description.sdp()));

                    unwrap!(
                        vdom_on_next_tick
                            .with_component({
                                let data_channel_clone = data_channel.clone();
                                move |root| {
                                    let wrtc = Self::get_web_rtc_data_from_root_render(root);
                                    wrtc.web_rtc_send_answer(local_description.sdp());
                                    wrtc.set_rtc_data_channel (data_channel_clone);
                                }
                            })
                            .await
                    );
                }
            });
        }
    }

    /// receive answer
    fn web_rtc_receive_answer(&mut self, vdom: VdomWeak, sdp: String) {
        websysmod::debug_write("web_rtc_receive_answer");
        let pc = self.get_rtc_peer_connection();
        spawn_local({
            async move {
                let vdom_on_next_tick = vdom.clone();
                let mut init_answer = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
                //set_sdp with a wrong name
                init_answer.sdp(&sdp);
                let _result =
                    wasm_bindgen_futures::JsFuture::from(pc.set_remote_description(&init_answer))
                        .await;
                //websysmod::debug_write(&format!("result set_remote_description: {:?}", &r));
                unwrap!(
                    vdom_on_next_tick
                        .with_component({
                            move |root| {
                                let wrtc = Self::get_web_rtc_data_from_root_render(root);
                                wrtc.set_rtc_accepted_call (true);
                                wrtc.web_rtc_send_ice_candidates();
                            }
                        })
                        .await
                );
            }
        });
    }

    /// this is candidate for the local object (not remote)
    fn web_rtc_setup_on_local_ice_candidate(vdom: VdomWeak, pc: &mut RtcPeerConnection) {
        let event_handler = Box::new(move |event: JsValue| {
            let vdom_on_next_tick = vdom.clone();
            websysmod::debug_write("on local ice_candidate");
            //send the candidate to the remote peer over ws
            let event = unwrap!(event.dyn_into::<RtcPeerConnectionIceEvent>());
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
                                    let wrtc = Self::get_web_rtc_data_from_root_render(root);
                                    //save to queue
                                    wrtc.get_mut_rtc_ice_queue().push(candidate_json);
                                    if wrtc.get_rtc_accepted_call() == true {
                                        wrtc.web_rtc_send_ice_candidates();
                                    }
                                }
                            })
                            .await
                    );
                });
            }
        });
        let cb_oh: Closure<dyn Fn(JsValue)> = Closure::wrap(event_handler);
        pc.set_onicecandidate(Some(cb_oh.as_ref().unchecked_ref()));
        //websysmod::debug_write(&format!("web_rtc_setup_on_local_ice_candidate: {}", ""));
        cb_oh.forget();
    }

    /// receive ice candidate
    fn web_rtc_receive_ice_candidate(&mut self, _vdom: VdomWeak, sdp: String) {
        let pc = self.get_rtc_peer_connection();
        spawn_local(async move {
            websysmod::debug_write("web_rtc_receive_ice_candidate");
            let ice_candidate: IceCandidate = unwrap!(serde_json::from_str(&sdp));
            let mut ice_candidate_init = RtcIceCandidateInit::new("");
            ice_candidate_init.candidate(&ice_candidate.candidate);
            ice_candidate_init.sdp_mid(Some(unwrap!(ice_candidate.sdp_mid).as_str()));
            ice_candidate_init.sdp_m_line_index(ice_candidate.sdp_m_line_index);
            //websysmod::debug_write(&format!("ice_candidate_init: {:?}", &ice_candidate_init));
            let ice_candidate = RtcIceCandidate::new(&ice_candidate_init);
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
    fn web_rtc_send_chat(&mut self, vdom: VdomWeak) {
        let web_rtc_chat_text =
            websysmod::get_input_element_value_string_by_id("web_rtc_chat_text");
        let dc = self.get_rtc_data_channel();
        match dc.ready_state() {
            RtcDataChannelState::Connecting => {
                websysmod::debug_write("Connection not open; queueing: ");
            }
            RtcDataChannelState::Open => {
                unwrap!(dc.send_with_str(&web_rtc_chat_text));
                //save to chat vector
                self.get_mut_rtc_chat().push(ChatMessage {
                    time: 0,
                    sender: 1,
                    msg: web_rtc_chat_text,
                })
            }
            RtcDataChannelState::Closing => {
                websysmod::debug_write("Attempted to send message while closing: ");
            }
            RtcDataChannelState::Closed => {
                websysmod::debug_write("Error! Attempt to send while connection closed.");
            }
            _ => {
                websysmod::debug_write("Some other ready_state ??.");
            }
        }
        vdom.schedule_render();
    }

    /// receive msg
    fn web_rtc_receive_chat(vdom: VdomWeak, text: String) {
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
                                let wrtc = Self::get_web_rtc_data_from_root_render(root);
                                wrtc.get_mut_rtc_chat().push(ChatMessage {
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
}
