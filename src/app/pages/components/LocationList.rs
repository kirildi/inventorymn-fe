#![allow(non_snake_case, unused)]
use crate::app::pages::LocationsPage::Location;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Props)]
pub struct LocationProps {
    location_list: Vec<Location>,
}

pub fn LocationList(props: LocationProps) -> Element {
    rsx! {
        ul {
            class: "w-full",
            for lc in props.location_list{
                li {
                    id: "{lc.location_id}",
                    class: "locations__wrapper flex flex-nowrap items-center w-full h-16 mb-4 rounded-md bg-neutral-800 border-2",
                    div {
                        class: "location__name w-2/3 self-start h-auto p-4 border-r-2",
                        "{lc.location_name}"
                    },
                    div {
                        class: "location__capacity w-1/3 self-center text-center h-auto p-4",
                        "{lc.location_capacity}"
                    }
                }
            }
        }
    }
}
