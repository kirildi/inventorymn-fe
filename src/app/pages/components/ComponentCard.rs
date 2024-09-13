#![allow(non_snake_case, unused)]
use crate::app::pages::ComponentsPage::Component;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Props)]
pub struct ComponentProps {
    component: Component,
}
pub fn ComponentCard(props: ComponentProps) -> Element {
    rsx! {
        div {
            class: "relative flex flex-row flex-wrap w-48 h-52 p-4 rounded-xl bg-zinc-900",
            div {
                class: "component__card__header w-40 h-32",
                img {
                    class:
                    if(props.component.component_image == ""){ "w-40 h-32 border border-white" }
                    else {"w-40 h-32"},
                    src: "{props.component.component_image}",
                    alt: "No img"
                }
            }
            div {
                class: "component__card__footer w-40 h-12 py-4 overflow-hidden text-ellipsis",
                span {
                    class: "",
                    "{props.component.component_name}"
                }
            }
        }
    }
}
