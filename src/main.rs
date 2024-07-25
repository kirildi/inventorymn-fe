#![allow(non_snake_case)]

use dioxus::prelude::*;
pub mod app;

use app::App::App;

const _TAILWIND_URL: &str = manganis::mg!(file("public/assets/css/tailwind.css"));
const _STYLES_URL: &str = manganis::mg!(file("public/assets/css/styles.css"));

fn main() {
    launch(App);

    // Load stylesheet with  LaunchBuilder with config
    //let index_html = format!(
    //         r#"
    // <!DOCTYPE html>
    // <html>
    //   <head>
    //     <title>InventoryMN</title>
    //     <meta content="text/html;charset=utf-8" http-equiv="Content-Type">
    //     <meta name="viewport" content="width=device-width, initial-scale=1.0">
    //     <meta charset="UTF-8">
    //     <link rel="stylesheet" href=".{}">
    //     <link rel="stylesheet" href=".{}">
    //   </head>
    //   <body>
    //     <div id="main"></div>
    //   </body>
    // </html>
    //         "#,
    //         _TAILWIND_URL, _STYLES_URL
    //     );
    //     LaunchBuilder::desktop()
    //         .with_cfg(dioxus::desktop::Config::new().with_custom_index(index_html))
    //         .launch(App);
}
