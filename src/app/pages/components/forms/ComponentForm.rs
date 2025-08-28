#![allow(non_snake_case, unused)]

use dioxus_elements::form;
use serde_json::json;
use std::collections::HashMap;

use dioxus::prelude::*;

pub fn ComponentForm() -> Element {
    let api_client: Signal<reqwest::Client> = use_context::<Signal<reqwest::Client>>();
    let mut component_name = use_signal(|| String::from(""));
    let mut component_description = use_signal(|| String::from(""));
    let mut component_image = use_signal(|| String::from(""));
    let mut component_vendor = use_signal(|| String::from(""));
    let mut install_date = use_signal(|| String::from(""));
    let mut quantity = use_signal(|| String::from("1"));

    let mut create_component = move |event: Event<FormData>| {
        let mut form_data = HashMap::new();
        form_data.insert(String::from("component_name"), component_name());
        form_data.insert(
            String::from("component_description"),
            component_description(),
        );
        form_data.insert(String::from("component_image"), component_image());
        form_data.insert(String::from("component_vendor"), component_vendor());
        form_data.insert(String::from("installed"), String::from(""));
        form_data.insert(String::from("status"), String::from(""));
        form_data.insert(String::from("install_date"), String::from(""));
        form_data.insert(String::from("create_date"), String::from(""));
        form_data.insert(
            String::from("user_id"),
            String::from("b845f7a7-cac0-4879-9ea2-bf685cdf7259"),
        );
        form_data.insert(
            String::from("project_id"),
            String::from("00000000-0000-0000-0000-000000000000"),
        );
        form_data.insert(
            String::from("location_id"),
            String::from("00000000-0000-0000-0000-000000000000"),
        );
        form_data.insert(String::from("quantity"), quantity());

        spawn(async move {
            let mut result = api_client()
                .post("http://localhost:3000/component/create")
                .json(&form_data)
                .send()
                .await;

            let response = result.unwrap();
            tracing::info!("[CREATE_COMPONENT_RESPONSE] response is {:?}", response);
            tracing::info!("[CREATE_COMPONENT_FORM]  {:?}", form_data);

            match response.error_for_status() {
                Ok(_res) => {
                    tracing::info!("COMPONENT CREATED !!!");
                }
                Err(err) => tracing::warn!("BAD request !!!. {:?}", err),
            }
        });
    };

    rsx! {
        form {
            class: "flex flex-wrap gap-4 w-full",
            onsubmit: create_component,
            // First row
            div {
                class: "flex w-full justify-between",
                div {
                    class: "flex flex-col flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2 ",
                        r#for: "component_name",
                        "Component Name:"
                    },
                    input {
                        id: "component_name",
                        class: "w-64 h-12 rounded-lg appearance-none block py-3 px-4 mb-3 focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 text-base",
                        placeholder: "e.g. Resistor R10/R10K/R10M",
                        r#type: "text",
                        onchange: move |event| component_name.set(event.value()),
                        value: "{component_name}"
                    }
                },
                div {
                    class: "flex flex-col flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2",
                        r#for: "component_image",
                        "Component Image:"
                    },
                    input {
                        id: "component_image",
                        class: "w-64 h-12 rounded-lg appearance-none block py-3 px-4 mb-3 focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 text-base",
                        placeholder: "e.g. image (url)",
                        r#type: "text",
                        onchange: move |event| component_image.set(event.value()),
                        value: "{component_image}"
                    }
                },
            },
            // Second row
            // TODO quantity fields
            div {
                class: "flex w-full justify-between",
                div {
                    class: "flex flex-col flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2",
                        r#for: "project_id",
                        "to Project:"
                    },
                    input {
                        id: "project_id",
                        class: "w-64 h-12 rounded-lg appearance-none block py-3 px-4 mb-3 focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 text-base",
                        placeholder: "leave blank for None",
                        r#type: "text",
                    }
                },
                div {
                    class: "flex flex-col flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2",
                        r#for: "component_vendor",
                        "Source/Vendor:"
                    },
                    input {
                        id: "component_vendor",
                        class: "w-64 h-12 rounded-lg appearance-none block py-3 px-4 mb-3 focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 text-base",
                        placeholder: "leave blank for None",
                        r#type: "text",
                        onchange: move |event| component_vendor.set(event.value()),
                        value: "{component_vendor}"
                    }
                },
            },
            // Third row
            div {
                class: "flex w-full justify-between",
                div {
                    class: "flex flex-col flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2",
                        r#for: "location_id",
                        "to Location:"
                    },
                    input {
                        id: "location_id",
                        class: "w-64 h-12 rounded-lg appearance-none block py-3 px-4 mb-3 focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 text-base",
                        placeholder: "leave blank for None",
                        r#type: "text",
                    }
                },
                div {
                    class: "flex flex-col flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2",
                        r#for: "installed_on",
                        "Installed On:"
                    },
                    input {
                        id: "installed_on",
                        class: "w-64 h-12 rounded-lg appearance-none block py-3 px-4 mb-3 focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 text-base",
                        r#type: "date",
                        value: "{install_date}"
                    }
                },
            },
            // Forth row
            div {
                class: "w-full",
                div {
                    class: "flex flex-col flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2",
                        r#for: "quantity",
                        "Quantity:"
                    },
                    input {
                        id: "quantity",
                        class: "w-64 h-12 rounded-lg appearance-none block py-3 px-4 mb-3 focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 text-base",
                        r#type: "text",
                        value: "{quantity}",
                        onchange: move |event| quantity.set(event.value())
                    }
                },
                div {
                    class: "flex flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2 ",
                        r#for: "component_description",
                        "Description:"
                    },
                    textarea {
                        id: "component_description",
                        class: "w-full h-24 rounded-lg appearance-none block py-3 px-4 mb-3 text-lg leading-tight focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 ",
                        placeholder: "Additional info",
                        onchange: move |event| component_description.set(event.value()),
                        value: "{component_description}"
                    }
                }
            },
            // Fifth row - form submit buttons
            div {
                class: "flex gap-8 w-full items-center justify-center",
                input {
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
