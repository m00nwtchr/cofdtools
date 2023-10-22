use cofd_schema::book::Book;
use dioxus::prelude::*;
use dioxus_router::prelude::Router;

use crate::router::Route;

#[derive(Default)]
pub struct AppState {
	pub books: Vec<Book>,
}

pub fn App(cx: Scope) -> Element {
	use_shared_state_provider(cx, AppState::default);

	cx.render(rsx! {
		Router::<Route> {}
	})
}
