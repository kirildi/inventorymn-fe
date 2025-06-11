#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::app::pages::components::{LocationList::LocationList, SideNav::SideNav};

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Props)]
pub struct Location {
    pub location_id: Uuid,
    pub location_name: String,
    pub location_capacity: i64,
    pub user_id: Uuid,
}

pub fn LocationsPage() -> Element {
    let api_client: Signal<reqwest::Client> = use_context::<Signal<reqwest::Client>>();
    let mut locations_list: Signal<Vec<Location>> = use_signal(|| Vec::<Location>::new());

    let mut request_data = HashMap::new();
    request_data.insert(
        String::from("user_id"),
        String::from("b845f7a7-cac0-4879-9ea2-bf685cdf7259"),
    );

    let fetch_locations = use_resource(move || {
        let value = request_data.clone();
        async move {
            let mut result = api_client()
                .get("http://localhost:3000/locations")
                .json(&value)
                .send()
                .await
                .unwrap()
                .json::<Vec<Location>>()
                .await;
            result
        }
    });

    let locations_wrapper: Result<VNode, RenderError> = match &*fetch_locations.read() {
        Some(Ok(response_location_list)) => {
            rsx! {
                LocationList { location_list: response_location_list.clone()}
            }
        }
        Some(Err(err)) => {
            tracing::warn!("BAD request !!!, {:?}", err);
            rsx! {
                h1 {
                    "Internal Error! No Locations found."
                }
            }
        }
        None => rsx! { h1 {"Loading locations..."} },
    };

    rsx! {
        section {
            class: "locations__section flex flex-row flex-wrap gap-4 w-3/4 h-[34rem] p-4 text-xl rounded-xl bg-zinc-800",
            {locations_wrapper}
        },
        section {
            class:"side__nav h-full w-1/4 text-xl rounded-xl",
            SideNav {},
            div {
                class: "",
                //TODO include StatusBox {}
            }
        }
    }
}
