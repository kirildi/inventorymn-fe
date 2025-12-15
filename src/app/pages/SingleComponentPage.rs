#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

use crate::app::pages::ComponentsPage::Component;

#[derive(PartialEq, Props, Clone)]
pub struct SingleComponentProps {
    name: String,
    component: Component,
}

pub fn SingleComponentPage(props: SingleComponentProps) -> Element {
    let component_image = match (props.component.component_image != "".to_string()) {
        true => rsx! {
            img {
                class: "w-64 h-64 rounded-xl",
                src: "{props.component.component_image}",
                alt: "Component image"
            }
        },
        false => rsx! {
            div {
                class: "w-full h-full flex justify-center items-center",
                i {
                    class: "ph ph-image-broken text-3xl"
                },
                h3 {
                    "Image unavailable"
                }
            }
        },
    };

    rsx!(
        div {
            class: "grid grid-rows-2 gap-4 bg-neutral-900 rounded-xl",
            div {
                class: "single__component__header grid grid-cols-[16rem_1fr] grid-rows-[repeat(minmax(0, 4rem))] w-full p-4",

                div {
                    class: "w-64 h-64 col-span-1 row-span-3 p-4 rounded-xl border-2 border-neutral-600",
                    {component_image}
                }

                div {
                    class: "single__component__title col-start-2 row-start-2 text-2xl h-16 p-4 pl-8 overflow-hidden whitespace-nowrap",
                    h2 {
                        class: "text-ellipsis",
                        "{props.name}"
                    }
                },
                div {
                    class: "single__component__controls col-start-2 row-start-3 p-4 pl-8 rounded-xl",
                    ul {
                        class: "flex flex-nowrap gap-4",
                        li {
                            class: "inline-block",
                            button {
                                class: "px-[0.4rem] py-[0.6rem] rounded-lg border-2 border-neutral-600",
                                value: "copy",
                                i { class:"ph ph-copy text-3xl" }
                            }
                        },
                        li {
                            class: "inline-block",
                            button {
                                class: "px-[0.4rem] py-[0.6rem] rounded-lg border-2 border-neutral-600",
                                value: "delete",
                                i { class: "ph ph-trash text-3xl" }
                            }
                        },
                    }
                },
            }
            div {
                class: "single__component__body w-full",
            }
        }
    )
}
