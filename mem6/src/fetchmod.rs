// fetchmod.rs
//! isolate/encapsulate fetch api in a module because it is all async
//region description
//! ## Async world
//! The coding style is "strange" because this is completely async. It is async because
//! JavaScript (the base of wasm) is limited to one single thread. And there is a lot to do
//! in this single thread if the thread is just waiting.
//! I wouldn't call this async, but "avoid-processor-waiting" coding.
//! A lot of promises (JavaScript) and futures (rust) here.
//! I am starting to miss good old events now.
//! The primary code flow starts typically in a mouse onclick event. When the async
//! function is called, that starts a secondary code flow that is completely
//! independent of the primary. The primary code flow will go immediately forward
//! and will not wait for the secondary. Usually there is no need for any code after
//! the async call in the primary. All the code now must be in the secondary code flow.
//! But the beginning part of the code is always the same, only the last part is different.
//! What about code reusing? So we must send a parameter that is a reference to a function to
//! be executed at the end. The world is upside down now. So confusing.
//! ## Promises, futures, Closures, reference to functions
//! Once upon a time programming was single threaded. It was easy to understand how the code flows.
//! From the primary flow you call a function and it returns (or not) something you can use in the primary flow.
//! Then came multicore processors. Now multithreading makes sense. From the primary code flow you spawn
//! a new thread (secondary flow) and do something in it. Hoping you will never need the result in the primary code flow.
//! That can complicate things a lot, because you never know when this result can come back.
//! Then came JavaScript that has only one thread. No multithreading there. But there is a lot of
//! waiting around for resources. So let invent async code on a single thread.
//! If you wait for something you pause this code flow and other code can run in that time.
//! After some time your code will continue as nothing happened in between. It is similar as multithread but on a single thread.
//! And it is never, never parallel, because it is single thread.
//! For this to work you don't send data around any more. You send the code that should be run in the future.
//! And here falls down all the experience of calling functions with data. All is reversed now. The world is upside down.
//! You cannot use "calling functions" any more. You cannot pass data in a normal way.
//! You cannot return values in a normal way.
//! Somebody is talking about async await syntax. I still await to see what problems will be there.
//! ## How to call this module and have a simple life
//! In the primary code call the `fetch_response` function as the last instruction:
//! `fetch_response(&vdom_weak,&request,&call_this_function_after_fetch);`
//! - `vdom_weak` is the main object of dodrio virtual dom. It contains RootRenderingComponent
//! that contains all the mutable data needed for rendering. And also the schedule_render function
//! we need after changing the data.
//! - `web_sys::Request` must be prepared with url, POST, Cors, body, headers,...
//! - `&call_this_function_after_fetch` is the reference to a function with specific signature.
//!
//! We are lucky because the `call_this_function_after_fetch` is just a normal function.
//! Nothing special there, except that the parameters must be of the same fixed types.
//! It can be coded in the old fashion of non-async programmers.
//! All the messy code is hidden and encapsulated inside fetchmod.rs with only one public function.
//! ## References
//! https://dev.to/werner/practical-rust-web-development-front-end-538d
//! https://rustwasm.github.io/docs/wasm-bindgen/examples/fetch.html
//endregion

//region: use
//use crate::logmod;
use crate::rootrenderingcomponentmod::RootRenderingComponent;

use unwrap::unwrap;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use web_sys::{Response};
use futures::Future;
//endregion

/// The only public function that starts the code flow around fetch_with_request()->Promise, text()->Promise  
/// This function returns nothing. All the code will be executed inside it.  
/// The last parameter is a reference to a (normal) function that will be executed at the end of this code flow.  
pub fn fetch_response(
    vdom_weak: dodrio::VdomWeak,
    request: &web_sys::Request,
    call_function_after_fetch: &'static (dyn for<'r> std::ops::Fn(
        &'r mut RootRenderingComponent,
        std::string::String,
    ) + 'static),
) {
    let window = unwrap!(web_sys::window());
    //1. wasm_bindgen knows only method fetch_with_request, and that returns a promise
    let request_promise = window.fetch_with_request(request);
    //transform promise into future
    let future = wasm_bindgen_futures::JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = unwrap!(resp_value.dyn_into());
            //the text() method returns a promise
            resp.text()
        })
        .and_then(|text_promise: js_sys::Promise| {
            // Convert this other `Promise` into a rust `Future`.
            wasm_bindgen_futures::JsFuture::from(text_promise)
        })
        .and_then(move |text_jsvalue| {
            let txt_response: String = unwrap!(text_jsvalue.as_string());
            //To change the data in rrc I must use the future `vdom.with_component`
            //it will be executed at the next tick to avoid concurrent data races.
            wasm_bindgen_futures::spawn_local(
                vdom_weak
                    .with_component({
                        move |root| {
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();

                            //and now at the end of the fetch Odyssey
                            //call the reference to the function passed as parameter
                            //The txt_response is captured by the Closure.
                            //This capture thing is so invisible and non intuitive.
                            //This is a catastrophe for readability and encapsulation.
                            //So non intuitive and non expressive. Where are good old parameters?

                            call_function_after_fetch(rrc, txt_response);
                        }
                    })
                    .map_err(|_| ()),
            );

            vdom_weak.schedule_render();

            // Send something back to JS as JsValue
            futures::future::ok(JsValue::from_str("ok"))
        });
    // future_to_promise() converts `Future` into `Promise` and schedules it to be executed
    wasm_bindgen_futures::future_to_promise(future);
}
