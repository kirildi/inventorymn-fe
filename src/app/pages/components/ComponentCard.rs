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
    let mut isInfoBoxVisible = use_signal(|| false);

    let toggleInfoBox = move |_| isInfoBoxVisible.set(!isInfoBoxVisible());
    rsx! {
        div {
            class: "component__wrapper flex flex-nowrap",
            div {
                class: format!("component__card relative flex flex-row flex-wrap w-48 h-52 px-4 pt-4 bg-zinc-900 border-neutral-500 {}",
            if isInfoBoxVisible() {"rounded-l-xl"} else {"rounded-xl"}),
                div {
                    class: "component__card__header w-40 h-28",
                    img {
                        class:
                        if(props.component.component_image == ""){ "w-40 h-28 border rounded-xl pl-8 pt-8 text-neutral-500 border-neutral-500" }
                        else {"w-40 h-28"},
                        src: "{props.component.component_image}",
                        alt: "No img"
                    }
                },
                div {
                    class: "component__card__footer flex flex-row w-40 h-12 pt-4",
                    span {
                        class: "component__name w-40 mr-2 overflow-hidden text-ellipsis",
                        "{props.component.component_name}"
                    },
                    button {
                        class: "w-8",
                        onclick: toggleInfoBox,
                        if isInfoBoxVisible() {
                            svg {
                                class: "svg__chevron__left size-6",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                path{
                                    stroke_linecap: "round",
                                    stroke_linejoin:"round",
                                    d: "M15.75 19.5 8.25 12l7.5-7.5",
                                }
                            }
                        }
                        else {
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
                            }
                        }
                    }
                }
            },
            div {
                class: format!("transition duration-700 ease-in component__info w-48 h-52 p-4 leading-8 bg-zinc-900 rounded-r-xl border-l-2 border-neutral-500 {}",
                if isInfoBoxVisible() {"visible"} else {"hidden"}),
                ul {
                    // li {"Assigned: {props.project.project_name}"},
                    // li {"Location: {props.location.location_name}"},
                    li {"Status: {props.component.status}"},
                    li {"Installed: {props.component.installed}"},
                    // li {"Category: {props.component.category}"},
                }
            }
        }
    }
}
