// statusdrinkmod.rs
//! code flow from this status

// region: use
use crate::*;
use dodrio::VdomWeak;
// endregion

/// on msg
pub fn on_msg_drink_end(
    _rrc: &mut RootRenderingComponent,
    _msg_sender_ws_uid: usize,
    _vdom: &VdomWeak,
) {
    websysmod::pause_music_player();
    htmltemplateimplmod::open_new_local_page("#p11");
}
