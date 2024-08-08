#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

pub fn UserPage() -> Element {
    rsx!(
        h1 {
            class: "text-2xl",
            "Hello, user"
        }
    )
}
