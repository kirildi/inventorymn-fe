#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

pub fn Header() -> Element {
    rsx!(
        header {
            div {
                class: "w-full h-8",
                span {
                    class: "text-lg",
                    "INVENTORY"
                },
                span{
                    class: "text-xs text-gray-400",
                    "MN"
                }
            },
            div {
                class: "w-12 h-8"
            }
        },
    )
}
