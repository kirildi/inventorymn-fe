#![allow(non_snake_case, unused)]
use crate::app::pages::ComponentsPage::Component;
use crate::router::PageRouter::Route;
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
            class: "component__wrapper relative flex flex-nowrap h-52",
            // Component Card
            div {
                class: format!("component__card relative flex flex-row flex-wrap w-48 px-4 pt-4 bg-neutral-800 hover:bg-neutral-700 border-neutral-600 {}",
                    if isInfoBoxVisible() {"rounded-l-xl"} else {"rounded-xl"}),
                Link {
                    class: "",
                    to: Route::SingleComponentPage { name: props.component.component_name.clone(), component: props.component.clone() },

                    div {
                        class: "component__card__header w-40 h-28 border-neutral-500",
                        if (props.component.component_image == ""){
                            div {
                                class: "w-40 h-28 flex justify-center items-center border rounded-xl text-neutral-500 border-neutral-500",
                                i {
                                    class: "ph ph-image-broken text-3xl"
                                },
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
                                i {
                                    class: "ph-fill ph-info"
                                }
                            }
                            else {
                                i {
                                    class: "ph ph-info"
                                }
                            }
                        }
                    }
                // }
            },
            // Component Card info box(hidden by default)
            div {
                class: format!("component__info absolute left-48 z-10 transition-opacity duration-300 ease-in w-48 h-52 p-4 leading-8 bg-neutral-800 rounded-r-xl border-l-2 border-neutral-700 {}",
                if isInfoBoxVisible() {"opacity-100 pointer-events-auto"} else {"opacity-0 pointer-events-none"}),
                ul {
                    // li {"Assigned: {props.project.project_name}"},
                    // li {"Location: {props.location.location_name}"},
                    li {"Status: {props.component.status}"},
                    li {"Installed: {props.component.installed}"},
                    li {"Description: {props.component.component_description}"},
                    // li {"Category: {props.component.category}"},
                }
            }
        }}
    }
}
