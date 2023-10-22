#![allow(non_snake_case)]

mod app;
mod components;
mod router;
mod views;

fn main() {
	dioxus_desktop::launch(app::App);
}
