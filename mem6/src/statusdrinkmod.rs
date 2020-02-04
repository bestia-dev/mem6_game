// statusdrinkmod.rs
//! code flow from this status

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::*;

use mem6_common::*;

use unwrap::unwrap;
//endregion

///on msg
pub fn on_msg_drink_end(rrc: &mut RootRenderingComponent, msg_sender_ws_uid: usize,vdom:&dodrio::VdomWeak) {
    fncallermod::open_new_local_page("#p11");
}
