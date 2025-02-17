#![allow(non_snake_case, unused)]

use dioxus::{
    desktop::{LogicalSize, WindowBuilder},
    prelude::*,
};
pub mod app;
pub mod router;

use app::App::App;

fn main() {
    const _DEFAULT_WIDTH: i32 = 1024;
    const _DEFAULT_HEIGHT: i32 = 680;
    const _DEFAULT_WINDOW_SIZE: LogicalSize<i32> =
        LogicalSize::new(_DEFAULT_WIDTH, _DEFAULT_HEIGHT);
    let title = String::from("InventoryMN");
    let window: WindowBuilder = dioxus::desktop::WindowBuilder::new()
        .with_inner_size(_DEFAULT_WINDOW_SIZE)
        .with_min_inner_size(_DEFAULT_WINDOW_SIZE)
        .with_title(title);

    LaunchBuilder::desktop()
        .with_cfg(dioxus::desktop::Config::new().with_window(window))
        .launch(App);
}
