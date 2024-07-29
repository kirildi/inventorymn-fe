#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

pub fn Header() -> Element {
    rsx!(
        header {
            class: "pt-8 px-8 pb-4",
            div {
                class: "w-auto h-8",
                span {
                    class: "text-xl",
                    "INVENTORY"
                },
                span{
                    class: "text-xs pl-0.5 text-gray-400",
                    "MN"
                }
            },
            div {
                class: "w-12 h-8"
            }
        },
    )
}
