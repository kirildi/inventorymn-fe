#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

pub fn ComponentsPage() -> Element {
    rsx!(
        h1 {
            class: "text-2xl",
            "All Components"
        }
    )
}
