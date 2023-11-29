#![allow(non_snake_case)]

mod app;
mod components;
mod router;
mod views;

#[cfg(not(target_arch = "wasm32"))]
static CSS: &str = include_str!("../public/tailwind.css");

fn main() {
	env_logger::init();

	#[cfg(not(target_arch = "wasm32"))]
	dioxus_desktop::launch_cfg(
		app::App,
		dioxus_desktop::Config::new().with_custom_head(format!("<style>{CSS}</style>")),
	);
}
