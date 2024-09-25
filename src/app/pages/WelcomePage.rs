#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

use crate::app::pages::components::forms::LoginForm::LoginForm;

pub fn WelcomePage() -> Element {
    rsx!(
        section {
            id: "login_section",
            class: "w-5/12 w h-96 border-r-2 border-zinc-300",
            LoginForm {},
        },
        section {
            id: "welcome_section",
            class: "flex flex-col flex-auto flex-wrap row-span-3 w-7/12 h-96 text-center",
            h1 {
                class: "pt-16 px-8 text-2xl font-semibold",
                "Explore our Inventory Management App!"
            },
            div {
                class: "w-full px-28",
                h3 {
                    class: "w-full py-12 px-8 text-xl font-light",
                    "Sign in to manage inventory or create an account to begin."
                }
            },
            div{
                class: "pt-12 px-24",
                button {
                    class: "w-36 h-12 rounded-full bg-violet-800",
                    "Sign Up"
                }
            }
        },
    )
}
