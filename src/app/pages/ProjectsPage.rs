#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::app::pages::components::{ProjectList::ProjectList, SideNav::SideNav};

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Props)]
pub struct Project {
    pub project_id: Uuid,
    pub project_name: String,
    pub project_description: String,
    pub project_image: String,
    pub create_date: String,
    pub finish_date: String,
    pub project_status: bool,
    pub project_progress: i64,
    pub user_id: Uuid,
}

pub fn ProjectsPage() -> Element {
    let api_client: Signal<reqwest::Client> = use_context::<Signal<reqwest::Client>>();
    let mut components_list: Signal<Vec<Project>> = use_signal(|| Vec::<Project>::new());

    let mut request_data = HashMap::new();
    request_data.insert(
        String::from("user_id"),
        String::from("b845f7a7-cac0-4879-9ea2-bf685cdf7259"),
    );

    let fetch_projects = use_resource(move || {
        let value = request_data.clone();
        async move {
            let mut result = api_client()
                .get("http://localhost:3000/projects")
                .json(&value)
                .send()
                .await
                .unwrap()
                .json::<Vec<Project>>()
                .await;
            result
        }
    });

    let projects_wrapper: Result<VNode, RenderError> = match &*fetch_projects.read() {
        Some(Ok(response_projects_list)) => {
            rsx! {
                ProjectList { project_list: response_projects_list.clone()}
            }
        }
        Some(Err(err)) => {
            tracing::warn!("BAD request !!!, {:?}", err);
            rsx! {
                h1 {
                    "Internal Error! No Projects found."
                }
            }
        }
        None => rsx! { h1 {"Loading projects..."} },
    };

    rsx! {
        section {
            class: "projects__section flex flex-row flex-wrap gap-4 w-3/4 h-[34rem] p-4 text-xl rounded-xl bg-zinc-800",
            {projects_wrapper}
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
