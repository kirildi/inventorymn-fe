#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CreateButtonProps {
    label: String,
}

pub fn CreateButton(props: CreateButtonProps) -> Element {
    rsx! {
        button {
            class: "w-20 h-16 rounded-r-xl text-center border-l-2 border-l-indigo-400 bg-indigo-700 hover:bg-indigo-900",
            "+",
        }
    }
}
