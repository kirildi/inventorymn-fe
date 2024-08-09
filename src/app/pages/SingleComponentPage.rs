#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

pub fn SingleComponentPage() -> Element {
    rsx!(
        h1 {
            class: "text-2xl",
            "Single Component"
        }
    )
}
