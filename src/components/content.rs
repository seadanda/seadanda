#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Content(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "flex w-full space-x-6 justify-center",

            a {
                href: "https://github.com/seadanda",
                target: "_blank",
                img {
                    src: "github.svg",
                    alt: "GitHub profile",
                    class: "h-16",
                }
            }

            a {
                href: "https://linkedin.com/in/donalomurray",
                target: "_blank",
                img {
                    src: "linkedin.svg",
                    alt: "LinkedIn profile",
                    class: "h-16",
                }
            }
        }
    ))
}
