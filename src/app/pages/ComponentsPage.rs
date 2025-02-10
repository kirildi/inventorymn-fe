#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::app::pages::components::{ComponentCard::ComponentCard, SideNav::SideNav};

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Props)]
pub struct Component {
    pub component_id: Uuid,
    pub component_name: String,
    pub component_description: String,
    pub component_image: String,
    pub installed: bool,
    pub status: String,
    pub install_date: String,
    pub create_date: String,
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub location_id: Uuid,
}

pub fn ComponentsPage() -> Element {
    let api_client: Signal<reqwest::Client> = use_context::<Signal<reqwest::Client>>();
    let mut components_list: Signal<Vec<Component>> = use_signal(|| Vec::<Component>::new());

    let mut request_data = HashMap::new();
    request_data.insert(
        String::from("user_id"),
        String::from("b845f7a7-cac0-4879-9ea2-bf685cdf7259"),
    );

    let fetch_components = use_resource(move || {
        let value = request_data.clone();
        async move {
            let mut result = api_client()
                .get("http://localhost:3000/components")
                .json(&value)
                .send()
                .await
                .unwrap()
                .json::<Vec<Component>>()
                .await;
            result
        }
    });

    let components_wrapper: Result<VNode, RenderError> = match &*fetch_components.read() {
        Some(Ok(_res)) => {
            rsx! {
                for cc in _res {
                    ComponentCard { component: cc.clone()}
                }
            }
        }
        Some(Err(err)) => {
            tracing::warn!("BAD request !!!, {:?}", err);
            rsx! {
                h1 {
                    "No Components found."
                }
            }
        }
        None => rsx! {"Loading components..."},
    };

    rsx! {
        section {
            class: "components__section flex flex-row flex-wrap gap-4 w-3/4 h-[34rem] p-4 text-xl rounded-xl bg-zinc-800",
            {components_wrapper}
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
