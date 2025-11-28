#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

use crate::app::pages::ComponentsPage::Component;

#[derive(PartialEq, Props, Clone)]
pub struct SingleComponentProps {
    name: String,
    component: Component,
}

pub fn SingleComponentPage(props: SingleComponentProps) -> Element {
    rsx!(
        div {
            class: "",
            div {
                class: "single__component__header flex justify-around text-2xl w-full h-16 p-4 bg-zinc-700",
                div {
                    class:"single__component__title",
                    h2 {
                        "{props.name}"
                    }
                    div {
                        class: "single__component__controls",
                        ul {
                            li { },
                            li { },
                        }
                    },
                }
            }
            div {
                class: "single__component__body w-full",
            }
        }
    )
}
