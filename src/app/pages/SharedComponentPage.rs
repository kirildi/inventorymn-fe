#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

use crate::router::PageRouter::Route;

pub fn SharedComponentPage() -> Element {
    rsx! { Outlet::<Route> {} }
}
