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
            class: "component__wrapper flex flex-nowrap h-52",
            div {
                class: format!("component__card relative flex flex-row flex-wrap w-48 px-4 pt-4 bg-neutral-800 border-neutral-500 {}",
                    if isInfoBoxVisible() {"rounded-l-xl"} else {"rounded-xl"}),
                div {
                    class: "component__card__header w-40 h-28 border-neutral-500",
                    if (props.component.component_image == ""){
                        div {
                            class: "w-40 h-28 border rounded-xl text-neutral-500 border-neutral-500",
                            svg {
                                class: "svg__no__img w-40 h-28 rounded-xl",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 12 6",
                                rect {
                                    class: "fill-neutral-900",
                                    x: "25%",
                                    y: "5%",
                                    rx: "1",
                                    width: "50%",
                                    height: "90%",
                                },
                                path {
                                    class: "stroke-purple-400",
                                    stroke_width: "0.1px",
                                    stroke_linecap: "round",
                                    stroke_linejoin:"round",
                                    d: "M4 1.4 L8 4.5z",
                                },
                                path {
                                    class: "stroke-purple-400",
                                    stroke_width: "0.1px",
                                    stroke_linecap: "round",
                                    stroke_linejoin:"round",
                                    d: "M8 1.4 L4 4.5z",
                                }
                            }
                        }
                    }
                    else {
                        img {
                            class: "w-40 h-28 border rounded-xl",
                            src: "{props.component.component_image}",
                            alt: "Component img"
                        }
                    },
                },
                div {
                    class: "component__card__footer flex flex-row w-40 h-12 pt-4",
                    span {
                        class: "component__name w-40 mr-2 overflow-hidden text-ellipsis text-purple-400",
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
                                path {
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
                class: format!("transition duration-500 ease-in component__info w-48 h-52 p-4 leading-8 bg-neutral-800 rounded-r-xl border-l-2 border-neutral-700 {}",
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
