// divnicknamemod.rs
//! loadand save nickname

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;

//use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
use unwrap::unwrap;
use wasm_bindgen::JsCast;
use futures::Future;
//use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.
//endregion

///render the nickname input
pub fn div_nickname_input<'b>(
    rrc: & RootRenderingComponent,
    bump: &'b Bump,
) -> Node<'b>
{
    //if the user did not yet input his nickname than blink
    //all the code is the same except the class and the call to schedule_render
    if rrc.game_data.my_nickname == "nickname" {
        dodrio!(bump,
        <div style="margin-left: auto ;margin-right: auto ;text-align: center" >
            <label>
                {vec![text(
                    bumpalo::format!(in bump, "{}",
                    "Write your nickname:")
                    .into_bump_str()
                )]}
                <input
                id="input_nickname"
                name="nickname"
                class="input_nickname_blink"
                value={bumpalo::format!(in bump, "{}",
                    rrc.game_data.my_nickname)
                    .into_bump_str()
                }
                onkeyup={ move |root, vdom_weak, event| {
                    //save on every key stroke
                    let v2 = vdom_weak.clone();
                    save_nickname_to_localstorage(&v2);
                    v2.schedule_render();
                    }
                }>
                </input>
            </label>
        </div>
        )
    } else {
        //if the use already has input his nickname no blinking is needed
        dodrio!(bump,
        <div style="margin-left: auto ;margin-right: auto ;text-align: center" >
            <label>
                {vec![text(
                    bumpalo::format!(in bump, "{}",
                    "Write your nickname:")
                    .into_bump_str()
                )]}
                <input
                id="input_nickname"
                name="nickname"
                value={bumpalo::format!(in bump, "{}",
                    rrc.game_data.my_nickname)
                    .into_bump_str()
                }
                onkeyup={ move |root, vdom_weak, event| {
                    //save on every key stroke
                    let v2 = vdom_weak.clone();
                    save_nickname_to_localstorage(&v2);
                    }
                }>
                </input>
            </label>
        </div>
        )
    }
}

///save nickname from html input elements to local storage and rrc
pub fn save_nickname_to_localstorage(vdom_weak: &dodrio::VdomWeak) {
    let window = unwrap!(web_sys::window(), "window");
    let document = unwrap!(window.document(), "document");

    //logmod::debug_write("before get_element_by_id");
    let input_nickname = unwrap!(document.get_element_by_id("input_nickname"));
    //logmod::debug_write("before dyn_into");
    let input_html_element_nickname = unwrap!(
        input_nickname.dyn_into::<web_sys::HtmlInputElement>(),
        "dyn_into"
    );
    //logmod::debug_write("before value()");
    let nickname_string = input_html_element_nickname.value();
    //logmod::debug_write("before as_str");
    let nickname = nickname_string.as_str();
    //logmod::debug_write(nickname);

    let ls = unwrap!(unwrap!(window.local_storage()));
    let _x = ls.set_item("nickname", nickname);

    //To change the data in rrc I must use the future `vdom.with_component`
    //it will be executed at the next tick to avoid concurrent data races.
    wasm_bindgen_futures::spawn_local(
        vdom_weak
            .with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    rrc.game_data.my_nickname = nickname_string;
                }
            })
            .map_err(|_| ()),
    );
}

///load nickname from local storage
pub fn load_nickname() -> String {
    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.local_storage()));
    let empty1 = "nickname".to_string();
    //return nickname
    unwrap!(ls.get_item("nickname")).unwrap_or(empty1)
}
