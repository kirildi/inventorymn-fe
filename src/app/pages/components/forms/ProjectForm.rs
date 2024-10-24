#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use serde_json::json;
use std::collections::HashMap;

pub fn ProjectForm() -> Element {
    let api_client: Signal<reqwest::Client> = use_context::<Signal<reqwest::Client>>();
    let mut project_name = use_signal(|| String::from(""));
    let mut project_image = use_signal(|| String::from(""));
    let mut project_description = use_signal(|| String::from(""));
    let mut project_status = use_signal(|| String::from("IN PROGRESS"));

    let mut create_project = move |_| {
        let mut form_data = HashMap::new();
        form_data.insert(String::from("project_name"), project_name());
        form_data.insert(String::from("project_image"), project_image());
        form_data.insert(String::from("project_description"), project_description());
        form_data.insert(
            String::from("user_id"),
            String::from("b845f7a7-cac0-4879-9ea2-bf685cdf7259"),
        );
        form_data.insert(
            String::from("location_id"),
            String::from("00000000-0000-0000-0000-000000000000"),
        );

        let json_data = serde_json::to_string(&form_data).unwrap();

        spawn(async move {
            let mut result = api_client()
                .post("http://localhost:3000/projects/create")
                .json(&json_data)
                .send()
                .await;
            tracing::info!("[PROJECTS_RESPONSE] result is {:?}", result);
            tracing::info!("[PROJECTS_FORM_DATA]  {:?}", form_data);

            match result.unwrap().error_for_status() {
                Ok(_res) => {
                    tracing::info!("PROJECT CREATED !!!")
                }
                Err(err) => tracing::warn!("[PROJECT]]BAD request !!!"),
            }
        });
    };

    rsx! {
        form {
            class: "flex flex-wrap gap-4 w-full",
            // First row
            div {
                class: "flex w-full justify-between",
                div {
                    class: "flex flex-col flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2 ",
                        r#for: "project_name",
                        "Project Name:"
                    },
                    input {
                        id: "project_name",
                        class: "w-64 h-12 rounded-lg appearance-none block py-3 px-4 mb-3 focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 text-base",
                        placeholder: "Name",
                        r#type: "text",
                        onchange: move |event| project_name.set(event.value()),
                        value: "{project_name}"
                    }
                },
                div {
                    class: "flex flex-col flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2",
                        r#for: "project_image",
                        "Project image"
                    },
                    input {
                        id: "project_image",
                        class: "w-64 h-12 rounded-lg appearance-none block py-3 px-4 mb-3 focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 text-base",
                        placeholder: "Image URL",
                        r#type: "text",
                        onchange: move |event| project_image.set(event.value()),
                        value: "{project_image}"
                    }
                },
            },
            // Second row
            div {
                class: "flex w-full justify-between",
                div {
                    class: "flex flex-col flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2",
                        r#for: "project_status",
                        "Project Status"
                    },
                    select {
                        id: "project_status",
                        prevent_default: "onchange",
                        class: "w-64 h-12 rounded-lg appearance-none block py-3 px-4 mb-3 focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 text-base",
                        onchange: move |event| project_status.set(event.value()),
                        value: "{project_status}",
                        option {
                            value: "IN PROGRESS",
                            "IN PROGRESS"
                        },
                        option {
                            value: "COMPLETED",
                            "COMPLETED"
                        },
                        option {
                            value: "RETIRED",
                            "RETIRED"
                        },
                        option {
                            value: "IDEA",
                            "IDEA"
                        },
                    }
                },
                // TODO: Parent project select field
            }
            // Third row
            div {
            class: "w-full",
                div {
                    class: "flex flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2 ",
                        r#for: "project_description",
                        "Description:"
                    },
                    textarea {
                        id: "project_description",
                        class: "w-full h-24 rounded-lg appearance-none block py-3 px-4 mb-3 text-lg leading-tight focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 ",
                        placeholder: "Additional info",
                        onchange: move |event| project_description.set(event.value()),
                        value: "{project_description}"
                    }
                }
            },
            // Fourth row - form submit buttons
            div {
                class: "flex gap-8 w-full items-center justify-center",
                input {
                    prevent_default: "onclick",
                    onclick: create_project,
                    class: "w-24 h-16 p-4 rounded-lg bg-zinc-900 cursor-pointer",
                    r#type: "submit",
                    "Create"
                },
                input {
                    class: "w-24 h-16 p-4 rounded-lg bg-zinc-900 cursor-pointer",
                    r#type: "reset",
                    "Clear"
                },
            }
        }
    }
}
