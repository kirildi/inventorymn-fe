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
            class:"h-full w-1/4 p-4 text-xl rounded-xl",
            "Various Stats"
        }
    )
}
