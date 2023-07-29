#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Hero(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            "Hero"
        }
    ))
}
