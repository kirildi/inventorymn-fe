#![allow(non_snake_case, unused)]

use crate::app::pages::components::forms::*;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CreateButtonProps {
    label: String,
}

pub fn CreateButton(props: CreateButtonProps) -> Element {
    let api_client: Signal<reqwest::Client> = use_context::<Signal<reqwest::Client>>();

    let mut is_create_pressed = use_signal(|| false);
    let form_name = format!("{}Form", props.label);
    let load_form_with_name = |f_name: String| -> Element {
        match f_name.as_str() {
            "LocationForm" => rsx! { LocationForm::LocationForm {}},
            "ProjectForm" => rsx! { ProjectForm::ProjectForm {}},
            _ => rsx! { ComponentForm::ComponentForm {}},
        }
    };

    let handleCreate = move |_| is_create_pressed.set(!is_create_pressed());

    rsx! {
        if is_create_pressed() {
            div {
                class: "fixed left-0 top-0 w-full h-full z-50 bg-black opacity-60",
                div {
                    class: "relative top-24 mx-24 z-50 w-80 h-60 rounded-xl bg-zinc-800",
                    {load_form_with_name(form_name)}
                },
                div {
                    onclick: handleCreate,
                    "Close",
                }
            }
        }

        button {
            onclick: handleCreate,
            class: "w-20 h-16 rounded-r-xl text-center border-l-2 border-l-indigo-400 bg-indigo-700 hover:bg-indigo-900",
            "+",
        }
    }
}
