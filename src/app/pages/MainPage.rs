#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

use crate::{
    app::{components::Header::Header, pages::components::SideNav::SideNav},
    router::PageRouter::Route,
};
pub fn MainPage() -> Element {
    rsx!(
        main {
            class: "flex flex-col h-screen max-h-screen p-8 pt-4 w-full",
            Header {},
            section {
                class: "grid grid-cols-[3fr_1fr] h-[80vh] max-h-[80vh] gap-x-8",
                Outlet::<Route> { }
                SideNav {},
            }
        }
    )
}
