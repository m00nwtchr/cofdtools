use std::path::PathBuf;

use dioxus::prelude::*;

use crate::{app::use_books, components::NavComponent};

pub fn Books(cx: Scope) -> Element {
	let files_uploaded: &UseRef<Vec<String>> = use_ref(cx, Vec::new);
	let books_context = use_books(cx);

	let upload = move |evt: Event<FormData>| {
		to_owned![files_uploaded];

		if let Some(file_engine) = &evt.files {
			let files = file_engine.files();
			files_uploaded.with_mut(|files_uploaded| {
				for file_name in files {
					cx.spawn({
						to_owned![books_context, file_name];
						async move {
							let book = cofd_miner::parse_book(PathBuf::from(file_name)).unwrap();

							let mut books = books_context.write();
							books.push(book);
							books.save();
						}
					});

					files_uploaded.push(file_name);
				}
			});
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
