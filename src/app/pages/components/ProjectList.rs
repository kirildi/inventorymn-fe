#![allow(non_snake_case, unused)]
use crate::app::pages::ProjectsPage::Project;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Props)]
pub struct ProjectProps {
    project_list: Vec<Project>,
}

pub fn ProjectList(props: ProjectProps) -> Element {
    rsx! {
        ul {
            class: "w-full",
            for prj in props.project_list{
                li {
                    id: "{prj.project_id}",
                    class: "projects__wrapper flex flex-nowrap items-center w-full h-16 mb-4 rounded-md bg-neutral-800 border",
                    div {
                        class: "project__name w-2/3 self-start h-auto p-4 border-r",
                        "{prj.project_name}"
                    },
                }
            }
        }
    }
}
