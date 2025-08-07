#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

use crate::{
    app::{components::Header::Header, pages::components::SideNav::SideNav},
    router::PageRouter::Route,
};
pub fn MainPage() -> Element {
    rsx!(
        main {
            class: "flex flex-col min-h-screen p-8 w-full",
            Header {},
            section {
                class: "grid grid-cols-[3fr_1fr] gap-x-8 flex-1",
                Outlet::<Route> { }
                SideNav {},
            }
        }
    )
}
