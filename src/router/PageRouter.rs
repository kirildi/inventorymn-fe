#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::app::pages::ComponentsPage::ComponentsPage;
use crate::app::pages::LocationsPage::LocationsPage;
use crate::app::pages::MainPage::MainPage;
use crate::app::pages::ProjectsPage::ProjectsPage;
use crate::app::pages::UserPage::UserPage;
use crate::app::pages::WelcomePage::WelcomePage;

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(MainPage)]
        #[route("/")]
        ComponentsPage {},
//        WelcomePage {},
        #[route("/user")]
        UserPage {},
        // #[route("/components")]
        // ComponentsPage {},
        // #[end_nest]
        #[route("/locations")]
        LocationsPage {},
        #[route("/projects")]
        ProjectsPage {},
        #[route("/:..route")]
            PageNotFound {
            route: Vec<String>,       
        },  
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            h1 { "Page not found" },
            p { "We are terribly sorry, but the page you requested doesn't exist." },
            p {
                pre {
                    color: "red",
                    "[LOG]:\n Attempted to navigate to: {route:?}"
                }
            },
            p {
                "Go back to: "
                Link {
                    to: "/".to_string(),
                    "home"
                }
            }
        }
    }
}
