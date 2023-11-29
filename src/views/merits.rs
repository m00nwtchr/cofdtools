use cofd_schema::{
	book::{Book, MeritItem},
	item::{merit::Merit, Item},
};
use dioxus::prelude::*;

use crate::{app::use_books, components::NavComponent};

pub fn Merits(cx: Scope) -> Element {
	let books = use_books(cx).read();
	let mut merits: Vec<&MeritItem> = books.iter().flat_map(|book| &book.merits).collect();

	merits.sort_by_key(|merit| &merit.name);
	let selected_merit = use_state(cx, || 0usize);

	let merit_rendered = if let Some(merit) = merits.get(*selected_merit.get()) {
		rsx!(span {
			h1 { "{ merit.name }" },
			for paragraph in &merit.description {
				p { class: "indent-4", "{ paragraph }" }
			}

			p { class: "indent-4",
				span { class: "font-bold", "Effects:" }, *merit.effects.first().unwrap_or_else(|| &String::new())
			}


			for paragraph in &merit.effects {
				p { class: "indent-4", "{ paragraph }" }
			}
		})
	} else {
		rsx!(span {})
	};

	let merits_rendered = merits.iter().enumerate().map(|(i, item)| {
		let dot_rating = item.inner.dot_rating.to_string();
		let prerequisites = item.inner.prerequisites.to_string();

		rsx!(tr {
			onclick: {
				move |_| selected_merit.set(i)
			},
			style: "cursor: pointer;",
			td { "{ item.name }" }
			td { "{ dot_rating }" }
			td { "{ prerequisites }" }
			td { "{ item.reference }" }
		})
	});

	cx.render(rsx! {
		header {
			NavComponent {}
		}
		main {
			class: "container mx-auto",

			div {
				class: "flex flex-row flex-wrap py-4",
				div {
					class: "w-full sm:w-2/3 md:w-3/4 pt-1 px-2",
					table {
						class: "w-full",
						tr {
							th {
								"Name"
							}
							th {
								"Rating"
							}
							th {
								"Prerequisites"
							}
							th {
								"Source"
							}
						}
						merits_rendered
					},
				}
				div {
					class: "w-full sm:w-1/3 md:w-1/4 px-2",
					merit_rendered
				}
			}
		}
	})
}
