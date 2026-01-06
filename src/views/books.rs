use dioxus::prelude::*;

use crate::{app::use_books, components::NavComponent};

#[component]
pub fn Books() -> Element {
	let mut files_uploaded = use_signal(Vec::new);
	let mut books_context = use_books();

	let upload = move |evt: Event<FormData>| {
		let files = evt.files();
		files_uploaded.with_mut(|files_uploaded| {
			for file in files {
				spawn({
					to_owned![file];
					async move {
						let task = tokio::task::spawn_blocking(move || {
							cofd_miner::parse_book(file.path()).unwrap()
						});

						let book = task.await.expect("Parsed book");
						let mut books = books_context.write();
						books.push(book);
						let _ = books.save();
					}
				});

				files_uploaded.push(file.name());
			}
		});
	};

	rsx! {
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
