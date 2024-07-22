#![allow(non_snake_case)]

use dioxus::prelude::*;

const _STYLE: &str = manganis::mg!(file("./public/tailwind.css"));

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(div {"Hello desktop app"})
}
