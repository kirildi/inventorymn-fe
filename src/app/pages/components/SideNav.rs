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
        div {
            class: "w-56 h-1/2 text-xl rounded-xl pt-0",
            for nav_button in nav_buttons {
                div {
                    class: "inline-flex",
                    div {
                        class: "w-40 h-16  p-4 mb-4 rounded-l-xl bg-indigo-900",
                        Link {
                            to: format!("/{}s", nav_button.to_lowercase()),
                            "{nav_button}",
                        }
                    },
                    CreateButton { label: nav_button.clone() }
                }
            }
        }
    )
}
