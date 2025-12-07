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
            class: "grid grid-rows-2 gap-4 bg-neutral-900 rounded-xl",
            div {
                class: "single__component__header grid grid-cols-[16rem_1fr] grid-rows-[repeat(minmax(0, 4rem))] w-full p-4",

                img {
                    class: "w-64 h-64 col-span-1 row-span-3 p-4 rounded-xl border-2 border-neutral-600",
                    src: "{props.component.component_image}",
                    alt: "Component image"
                },
                div {
                    class: "single__component__title col-start-2 row-start-2 text-2xl h-16 p-4 pl-8",
                    h2 {
                        class: "",
                        "{props.name}"
                    }
                },
                div {
                    class: "single__component__controls col-start-2 row-start-3 p-4 pl-8",
                    ul {
                        li { button { value: "copy"}},
                        li { button { value: "delete"}},
                    }
                },
            }
            div {
                class: "single__component__body w-full",
            }
        }
    )
}
