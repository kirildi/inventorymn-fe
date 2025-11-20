#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

use crate::app::pages::ComponentsPage::Component;

#[derive(PartialEq, Props, Clone)]
pub struct SingleComponentProps {
    pub name: String,
    // component: Component,
}

pub fn SingleComponentPage(props: SingleComponentProps) -> Element {
    rsx!(
        h1 {
            class: "text-2xl",
            "Single Component {props.name}"
        }
    )
}
