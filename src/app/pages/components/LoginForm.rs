#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

pub fn LoginForm() -> Element {
    rsx!(
        div {
            h1{
                "Sign in"
            },
            form {
                label {
                    "USERNAME"
                },
                input {}
                label {
                    "PASSWORD"
                },
                input {},
                button {"Sign In"}
            }
        }
    )
}
