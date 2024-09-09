#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

pub fn SideNav() -> Element {
    let nav_buttons: [String; 3] = [
        String::from("Components"),
        String::from("Projects"),
        String::from("Locations"),
    ];
    rsx!(
        div {
            class: "w-56 h-1/2 text-xl rounded-xl pt-0",
            for but in nav_buttons {
                div {
                    class: "inline-flex",
                    div {
                        class: "w-40 h-16  p-4 mb-4 rounded-l-xl bg-indigo-900",
                        "{but}",
                    }
                    button {
                        class: "w-20 h-16 rounded-r-xl text-center border-l-2 border-l-indigo-400 bg-indigo-700 hover:bg-indigo-900",
                        "+"
                    }
                }
            }
        }
    )
}