use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::router::Route;

pub fn NavComponent(cx: Scope) -> Element {
	render! {
		nav {
			Link { to: Route::Home {}, "Home" }
			Link { to: Route::Books {}, "Books" }
			Link { to: Route::Merits {}, "Merits" }
		}
	}
}
