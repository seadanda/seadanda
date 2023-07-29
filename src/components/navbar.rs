#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx!(
        nav {
            class: "w-full flex py-6 justify-between items-center",
            a {
                href: "/",
                img {
                    src: "default-monochrome.svg",
                    alt: "Seadanda Development",
                    class: "h-6",
                }
            }
            ul {
                class: "flex",
                li {
                    a {
                        href: "mailto:donalm@seadanda.dev",
                        class: "px-4 py-2 hover:text-gorm-500",

                        "Contact"
                    }
                }
            }
        }
    ))
}
