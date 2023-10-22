use dioxus::prelude::*;

use crate::components::NavComponent;

pub fn Home(cx: Scope) -> Element {
	render! {
		header {
			NavComponent {}
		}
		main {

		}
	}
}
