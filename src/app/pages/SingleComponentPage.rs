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
                class: "w-full h-full flex flex-col justify-center items-center",
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
            class: "grid gap-4 bg-neutral-900 rounded-xl",
            div {
                class: "single__component__header grid grid-cols-[1fr_12rem] grid-rows-[repeat(3,auto)] w-full p-8 pb-4",

                div {
                    class: "w-48 h-48 col-start-3 row-span-3 p-4 rounded-xl border-2 border-neutral-600",
                    {component_image}
                }

                div {
                    class: "single__component__title col-start-1 row-start-2 text-2xl p-4 overflow-hidden whitespace-nowrap",
                    h2 {
                        class: "text-ellipsis",
                        "{props.name}"
                    }
                },
                div {
                    class: "single__component__controls col-start-1 row-start-3 p-4 rounded-xl",
                    ul {
                        class: "flex flex-nowrap gap-4",
                        li {
                            class: "inline-block",
                            button {
                                class: "p-[0.4rem] rounded-lg border-2 border-neutral-600",
                                value: "copy",
                                i { class:"ph ph-copy text-2xl" }
                            }
                        },
                        li {
                            class: "inline-block",
                            button {
                                class: "p-[0.4rem] rounded-lg border-2 border-neutral-600",
                                value: "delete",
                                i { class: "ph ph-trash text-2xl" }
                            }
                        },
                    }
                },
            },
            div {
                class: "single__component__body w-full grid grid-rows-2 p-8 pt-4",
                div {
                    class: "component__info__list col-start-1 col-span-2 pb-8 flex gap-8 justify-evenly",
                    div {
                        class: "w-1/2 p-4 rounded-xl bg-neutral-950",
                        ul {
                            li { span {"Status: "}, { props.component.status }},
                            li { span {"Installed: "}, { props.component.installed.to_string()}},
                            li { span {"Install Date: "}, { props.component.install_date }},
                            li { span {"Create Date: "}, { props.component.create_date }},
                        }
                    },
                    div {
                        class: "rounded-xl bg-neutral-950 p-4",
                        ul {
                            li {span {"Project Name: "}, { props.component.project_id.to_string()}},
                            li {span {"Location Name: "},{ props.component.location_id.to_string()}},
                        }
                    },
                },
                div {
                    class: "component__description col-start-1 col-span-2 row-start-2 p-4 rounded-xl bg-neutral-950",
                    span {"Description: "},
                    { props.component.component_description }
                },
            }
        }
    )
}
