use std::path::PathBuf;

use dioxus::prelude::*;

use crate::{app::AppState, components::NavComponent};

pub fn Books(cx: Scope) -> Element {
	let files_uploaded: &UseRef<Vec<String>> = use_ref(cx, Vec::new);
	let app_state_context = use_shared_state::<AppState>(cx).unwrap();

	let upload = move |evt: Event<FormData>| {
		to_owned![files_uploaded];

		if let Some(file_engine) = &evt.files {
			let files = file_engine.files();
			for file_name in files {
				files_uploaded.write().push(file_name.clone());

				cx.spawn({
					to_owned![app_state_context];
					async move {
						let book = cofd_miner::parse_book(PathBuf::from(file_name)).unwrap();
						app_state_context.write().books.push(book);
						println!("Parsed book!");
					}
				});
			}
		}
	};

	render! {
		header {
			NavComponent {}
		}
		main {

			input {
				r#type: "file",
				accept: ".pdf",
				multiple: true,
				onchange: upload,
			},

			div { "progress: {files_uploaded.read().len()}" },

			ul {
				for file in files_uploaded.read().iter() {
					li { "{file}" }
				}
			}
		}
	}
}
