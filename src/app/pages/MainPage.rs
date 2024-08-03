#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

use crate::{app::components::Header::Header, router::PageRouter::Route};

pub fn MainPage() -> Element {
    rsx!(
        Header {},
        div {
            class: "flex flex-row m-8",
            Outlet::<Route> { }
        }
    )
}
