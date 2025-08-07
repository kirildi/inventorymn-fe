#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

use crate::{app::pages::components::CreateButton::CreateButton, router::PageRouter::Route};

pub fn SideNav() -> Element {
    let nav_buttons: [String; 3] = [
        String::from("Component"),
        String::from("Project"),
        String::from("Location"),
    ];

    rsx!(
        ul {
            class: "w-auto h-1/2 text-xl rounded-xl pt-0 sticky top-4",
            for nav_button in nav_buttons {
            li {
                class: "inline-flex mb-4 ",
                    Link {
                        class: "inline-block w-40 h-16 p-4 rounded-l-xl bg-purple-900 hover:bg-purple-800",
                        to: format!("/{}s", nav_button.to_lowercase()),
                        {format!("{}s", nav_button)},
                    },
                    CreateButton { label: nav_button.clone() }
                }
            }
        }
    )
}
