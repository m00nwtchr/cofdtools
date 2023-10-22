use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::views::*;

#[derive(Routable, Clone)]
pub enum Route {
	#[route("/")]
	#[redirect("/:..route", |route: Vec<String>| Route::Home {})]
	Home {},
	#[route("/books")]
	Books {},
	#[route("/merits")]
	Merits {},

	#[route("/:..route")]
	NotFound { route: Vec<String> },
}

#[inline_props]
fn NotFound(cx: Scope, route: Vec<String>) -> Element {
	render! {
		h1 { "Page not found" }
	}
}
