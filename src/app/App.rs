#![allow(non_snake_case, unused)]

use crate::app::components::Header::Header;
use crate::app::pages::WelcomePage::WelcomePage;
use crate::router::PageRouter::Route;
use dioxus::prelude::*;
use inventorymn_be::presentation::web_api::rest::RestService::RestService;
use reqwest;
use tracing::info;

pub fn App() -> Element {
    use_context_provider(|| Signal::new(reqwest::Client::new()));
    spawn(async {
        let handle = tokio::spawn(async move {
            let mut rest_service = RestService::init().await.unwrap();
            rest_service.serve().await;
        });
        // let result = handle.await.unwrap();
        // info!("[REST SERVICE] {:?}", result);
    });
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/public/assets/css/tailwind.css")
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/public/assets/css/styles.css")
        }
        document::Link {
            rel: "stylesheet",
            r#type: "text/css",
            href: "https://cdn.jsdelivr.net/npm/@phosphor-icons/web@2.1.1/src/regular/style.css"
        }
        div {
        class: "w-full h-full",
        Router::<Route> { }
    }}
}
