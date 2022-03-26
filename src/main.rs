#![allow(non_snake_case)]

use dioxus::prelude::*;
use web_sys::console;

fn main() {
    console::log_1(&"Hello, world!".into());
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "hello, wasm!" }
        Counter {}
        Uppercase { text: "well, hello there".into() }
    })
}

fn Counter(cx: Scope) -> Element {
    let count = use_state(&cx, || 0);

    cx.render(rsx! {
        div {
            button {
                class: "px-3 py-2 rounded-md text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-500 focus:outline-none focus:bg-indigo-700 focus:shadow-outline-indigo active:bg-indigo-700 transition ease-in-out duration-150",
                onclick: move |_| {
                    count.set(count.get() + 1);
                },
                "{count}"
            }
        }
    })
}

#[inline_props]
fn Uppercase(cx: Scope, text: String) -> Element {
    let transformed = text.to_uppercase();
    cx.render(rsx! {
        span {
            class: "text-xl font-bold text-red-500",
            "{transformed}"
        }
    })
}
