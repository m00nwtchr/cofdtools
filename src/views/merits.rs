use std::fmt::Display;

use cofd_schema::book::Book;
use cofd_schema::dot_range::DotRange;
use cofd_schema::item::{Item, ItemProp, PropValue};
use cofd_schema::prerequisites;
use dioxus::html::{td, tr};
use dioxus::prelude::*;

use crate::{app::AppState, components::NavComponent};

pub fn Merits(cx: Scope) -> Element {
	let app_state_context = use_shared_state::<AppState>(cx).unwrap().read();

	let mut merits: Vec<(&Book, &Item)> = app_state_context
		.books
		.iter()
		.flat_map(|book| book.merits.iter().map(move |m| (book, m)))
		.collect();

	merits.sort_unstable_by_key(|(_, merit)| &merit.name);

	let merits_rendered = merits.iter().map(|(book, item)| {
		match (
			item.properties.get(&ItemProp::DotRating),
			item.properties.get(&ItemProp::Prerequisites),
		) {
			(Some(PropValue::DotRange(rating)), Some(PropValue::Prerequisites(prerequisites))) => {
				let v = prerequisites
					.iter()
					.map(|p| p.to_string())
					.collect::<Vec<_>>()
					.join(", ");
				rsx!(tr {
					td { "{ item.name }" }
					td { "{ rating }" }
					td { "{ v }" }
					td { "{ book.info.short_name } pg.{ item.page }" }
				})
			}
			_ => rsx!(tr {}),
		}
	});

	cx.render(rsx! {
		header {
			NavComponent {}
		}
		main {
			"Merits"

			table {
				tr {
					th {
						"Name"
					}
					th {
						"Rating"
					}
					th {
						"Prerequisite"
					}
					th {
						"Source"
					}
				},
				merits_rendered
			}
		}
	})
}
