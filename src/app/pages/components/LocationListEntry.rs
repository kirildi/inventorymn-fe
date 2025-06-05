#![allow(non_snake_case, unused)]
use crate::app::pages::LocationsPage::Location;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Props)]
pub struct LocationProps {
    location: Location,
}

pub fn LocationListEntry(props: LocationProps) -> Element {
    rsx! {
        div {
            id: "{props.location.location_id}",
            class: "locations__wrapper flex flex-nowrap w-full h-16 mb-4 rounded-md bg-neutral-800 border-2",
            div {
                class: "location__name w-2/3 self-start h-auto p-4 border-r-2",
                "{props.location.location_name}"
            },
            div {
                class: "location__capacity w-1/3 self-center h-auto p-4",
                "{props.location.location_capacity}"
            }
        }
    }
}
