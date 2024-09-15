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
            class: "relative flex flex-row flex-wrap w-48 h-52 p-4 rounded-xl bg-zinc-900 border-neutral-500",
            div {
                class: "component__card__header w-40 h-28",
                img {
                    class:
                    if(props.component.component_image == ""){ "w-40 h-28 border text-neutral-500 border-neutral-500" }
                    else {"w-40 h-28"},
                    src: "{props.component.component_image}",
                    alt: "No img"
                }
            }
            div {
                class: "component__card__footer flex flex-row w-40 h-12 pt-4",
                span {
                    class: "component__name w-40 overflow-hidden text-ellipsis",
                    "{props.component.component_name}"
                }
                button {
                    class: "w-8",
                    svg {
                        class: "svg__chevron__right size-6",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "1.5",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "m8.25 4.5 7.5 7.5-7.5 7.5",
                        }
                    },
                    //   //TODO this is the "left" icon snippet, when info box opened, need to be implemented
                        // svg {
                        //     class: "svg__chevron__left size-6",
                        //     xmlns: "http://www.w3.org/2000/svg",
                        //     fill: "none",
                        //     view_box: "0 0 24 24",
                        //     stroke_width: "1.5",
                        //     stroke: "currentColor",
                        //     path{
                        //         stroke_linecap: "round",
                        //         stroke_linejoin:"round",
                        //         d: "M15.75 19.5 8.25 12l7.5-7.5",
                        //     }
                        // }

                }
            }
        }
    }
}
