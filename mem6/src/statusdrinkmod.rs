// statusdrinkmod.rs
//! code flow from this status

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::*;

//use unwrap::unwrap;
//endregion

///on msg
pub fn on_msg_drink_end(
    _rrc: &mut RootRenderingComponent,
    _msg_sender_ws_uid: usize,
    _vdom: &dodrio::VdomWeak,
) {
    fncallermod::open_new_local_page("#p11");
}
