#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

pub fn StatusBox() -> Element {
    rsx!(

        div {
            class:"h-full w-1/4 p-4 text-xl rounded-xl",
            "Various Stats"
        }
    )
}
