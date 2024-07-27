#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

use crate::app::pages::components::LoginForm::LoginForm;

pub fn WelcomePage() -> Element {
    rsx!(
        div {
            LoginForm {},
        },
        div {
            h1 {
                "Explore our Inventory Management App!"
            },
            h3 {
                "Sign in to manage inventory or create an account to begin."
            }
        },
        div{
            button {"Sign Up"}
        }
    )
}
