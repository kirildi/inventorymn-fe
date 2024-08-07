#![allow(non_snake_case, unused)]

use dioxus::{
    desktop::{LogicalSize, WindowBuilder},
    prelude::*,
};
pub mod app;
pub mod router;

use tracing::info;
use tracing::Level;

use app::App::App;

const _TAILWIND_URL: &str = manganis::mg!(file("public/assets/css/tailwind.css"));
const _STYLES_URL: &str = manganis::mg!(file("public/assets/css/styles.css"));

fn main() {
    // tracing_subscriber::fmt().init();

    const _DEFAULT_WIDTH: i32 = 1024;
    const _DEFAULT_HEIGHT: i32 = 680;
    const _DEFAULT_WINDOW_SIZE: LogicalSize<i32> =
        LogicalSize::new(_DEFAULT_WIDTH, _DEFAULT_HEIGHT);
    let title = String::from("InventoryMN");
    let window: WindowBuilder = dioxus::desktop::WindowBuilder::new()
        .with_inner_size(_DEFAULT_WINDOW_SIZE)
        .with_min_inner_size(_DEFAULT_WINDOW_SIZE)
        .with_title(title);

    // Load stylesheet with  LaunchBuilder with config
    let index_html = format!(
        r#"
    <!DOCTYPE html>
    <html>
        <head>
            <title>InventoryMN</title>
            <meta content="text/html;charset=utf-8" http-equiv="Content-Type">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <meta charset="UTF-8">
            <link rel="stylesheet" href="/dist{}">
            <link rel="stylesheet" href="/dist{}">
        </head>
        <body>
            <div id="main"></div>
        </body>
    </html>
    "#,
        _TAILWIND_URL, _STYLES_URL
    );
    LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(window)
                .with_custom_index(index_html),
        )
        .launch(App);
}
