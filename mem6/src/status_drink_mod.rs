// status_drink_mod.rs
//! code flow from this status

// region: use
use crate::*;

use unwrap::unwrap;
use dodrio::VdomWeak;
// endregion

/// on msg
pub fn on_msg_drink_end(
    _rrc: &mut RootRenderingComponent,
    _msg_sender_ws_uid: usize,
    _vdom: VdomWeak,
) {
    html_template_impl_mod::open_new_local_page("#p11");
}

/// play sound mp3. The audio element is on the html page
/// so when it closes also the sound stops.
pub fn play_sound_for_drink(rrc: &RootRenderingComponent) {
    // randomly choose a link from rrc.audio
    let num = websysmod::get_random(0, rrc.game_data.audio.len());
    // prepare the audio element with src filename of mp3
    #[allow(clippy::indexing_slicing)]
    // indexing cannot panic if the random num is created from 0..len()
    let src_mp3 = format!("audio/{}", rrc.game_data.audio[num]);
    let audio_element = websysmod::get_audio_element_by_id("audio");
    audio_element.set_src(&src_mp3);
    let _x = unwrap!(audio_element.play());
}
