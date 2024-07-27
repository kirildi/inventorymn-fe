#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

use crate::app::components::Header::Header;
use crate::app::pages::WelcomePage::WelcomePage;

pub fn App() -> Element {
    rsx!(
        div {
            class: "w-full h-full",
            Header {},
            WelcomePage {},

        }
    )
}
