#![allow(non_snake_case)]
mod components;

use components::{Content, Hero, Hiring, Navbar};
use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    // start app
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "min-h-screen flex flex-col w-full",

            // navbar
            div {
                class: "w-full px-4 bg-gradient-to-br from-corcra-900 via-corcra-500 to-gorm-500 text-white",
                Navbar {}
            }

            // Hero
            div {
                class: "w-full",
                Hero {}
            }

            // Content
            div {
                class: "w-full p-4 flex-grow",
                Content {}
            }

            // Hiring
            div {
                class: "mt-auto pb-4",

                div {
                    class: "mx-auto max-w-lg",
                    Hiring {}
                }
            }
        }
    ))
}
