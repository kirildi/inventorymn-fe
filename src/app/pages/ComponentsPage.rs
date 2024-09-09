#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use uuid::Uuid;

use crate::app::pages::components::SideNav::SideNav;

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
    let components = use_signal(|| Vec::<Component>::new());

    rsx!(
        section {
            class: "components__section h-full w-3/4 p-4 text-xl rounded-xl bg-zinc-800",
            if components.is_empty() {
                h1 {
                    "No Components found."
                }
            }
            else {

                //TODO include ComponentCard {}
            }

        },
        section {
            class:"side__nav h-full w-1/4 text-xl rounded-xl",
            SideNav {},
            div {
                class: "",
                //TODO include StatusBox {}
            }
        }
    )
}
