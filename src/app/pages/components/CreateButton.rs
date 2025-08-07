#![allow(non_snake_case, unused)]

use crate::app::pages::components::forms::*;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CreateButtonProps {
    label: String,
}

pub fn CreateButton(props: CreateButtonProps) -> Element {
    let mut is_create_pressed = use_signal(|| false);
    let form_name = format!("{}Form", props.label);
    let mut label = props.label;
    let load_form_with_name = |f_name: String| -> Element {
        match f_name.as_str() {
            "LocationForm" => rsx! { LocationForm::LocationForm {}},
            "ProjectForm" => rsx! { ProjectForm::ProjectForm {}},
            _ => rsx! { ComponentForm::ComponentForm {}},
        }
    };

    let toggle_create_form = move |_| is_create_pressed.set(!is_create_pressed());

    rsx! {
        if is_create_pressed() {
            div {
                class: "form__under__layer fixed left-0 top-0 w-full h-full z-50 bg-black bg-opacity-60",
                div {
                    class: "create__form__wrapper relative top-20 mx-24 z-50 rounded-xl bg-zinc-800",
                    div {
                        class: "form__header flex flex-row flex-nowrap p-4",
                        div {
                            class: "form__header__title w-full h-8",
                            "Crete new {label}:"
                        },
                        button {
                            class: "w-8 h-8" ,
                            onclick: toggle_create_form,
                            "X",
                        }
                    }
                    div {
                        class: "form__body flex p-4 w-full",
                        {load_form_with_name(form_name)}
                    },
                },

            }
        }

        button {
            onclick: toggle_create_form,
            class: "w-20 h-16 rounded-r-xl text-center bg-purple-700 hover:bg-purple-800",
            svg {
                class: "w-8 h-6 block mx-auto",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 512 512",
                width: "100%",
                height: "100%",
                fill: "currentColor",
                g {
                    path {
                        d: "M480,224H288V32c0-17.673-14.327-32-32-32s-32,14.327-32,32v192H32c-17.673,0-32,14.327-32,32s14.327,32,32,32h192v192   c0,17.673,14.327,32,32,32s32-14.327,32-32V288h192c17.673,0,32-14.327,32-32S497.673,224,480,224z"
                    }
                }
            }
        }
    }
}
