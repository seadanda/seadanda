#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Hiring(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "relative group",

            div {
                class: "absolute -inset-1 bg-gradient-to-r from-corcra-500 to-gorm-500 rounded-lg blur opacity-25 group-hover:opacity-100 transition duration-1000 group-hover:duration-200",
            }
            div {
                class: "relative px-7 py-6 bg-white ring-1 ring-gray-900/5 rounded-lg leading-none flex items-top justify-start space-x-6",
                div {
                    class: "space-y-2",
                    
                    p {
                        class: "text-slate-800",

                        "I'm a full-stack developer and technical leader."
                    }
                    a {
                        href: "mailto:donalm@seadanda.dev",
                        class: "block text-gorm-400 group-hover:text-gorm-800 pt-1 transition duration-200",
                        "Hire me →"
                    }
                }
            }
        }
    ))
}