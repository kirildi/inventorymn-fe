#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

use crate::app::components::Header::Header;

pub fn App() -> Element {
    rsx!(
        div { Header {} }
    )
}
