#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use serde_json::json;
use std::collections::HashMap;

pub fn LocationForm() -> Element {
    let api_client: Signal<reqwest::Client> = use_context::<Signal<reqwest::Client>>();
    let mut location_name = use_signal(|| String::from(""));
    let mut location_capacity = use_signal(|| String::from(""));

    let mut create_location = move |event: Event<MouseData>| {
        let mut form_data = HashMap::new();
        form_data.insert(String::from("location_name"), location_name());
        form_data.insert(String::from("location_capacity"), location_capacity());
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
                .post("http://localhost:3000/locations/create")
                .json(&json_data)
                .send()
                .await;
            tracing::info!("[LOCATIONS_RESPONSE] result is {:?}", result);
            tracing::info!("[LOCATIONS_FORM_DATA]  {:?}", form_data);

            match result.unwrap().error_for_status() {
                Ok(_res) => {
                    tracing::info!("LOCATION CREATED !!!")
                }
                Err(err) => tracing::warn!("[LOCATION]BAD request !!!"),
            }
        });
    };

    rsx! {
        form {
            class: "flex flex-wrap gap-4 w-full",
            div {
                class: "flex w-full justify-between",
                div {
                    class: "flex flex-col flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2 ",
                        r#for: "location_name",
                        "Location Name:"
                    },
                    input {
                        id: "location_name",
                        class: "w-64 h-12 rounded-lg appearance-none block py-3 px-4 mb-3 focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 text-base",
                        placeholder: "Name",
                        r#type: "text",
                        onchange: move |event| location_name.set(event.value()),
                        value: "{location_name}"
                    }
                },
                div {
                    class: "flex flex-col flex-wrap",
                    label {
                        class: "block uppercase tracking-wide text-xs font-bold mb-2",
                        r#for: "location_cap",
                        "Location capacity"
                    },
                    input {
                        id: "location_cap",
                        class: "w-64 h-12 rounded-lg appearance-none block py-3 px-4 mb-3 focus:outline-none focus:bg-zinc-900 placeholder-zinc-600 text-base",
                        placeholder: "Capacity",
                        r#type: "text",
                        onchange: move |event| location_capacity.set(event.value()),
                        value: "{location_capacity}"
                    }
                },
            }
            div {
                class: "flex gap-8 w-full items-center justify-center",
                input {
                    onclick: create_location,
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
