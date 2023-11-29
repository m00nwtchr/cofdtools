use std::{
	fs::File,
	ops::{Deref, DerefMut},
};

use anyhow::anyhow;
use cofd_schema::book::Book;
use dioxus::prelude::*;
use dioxus_router::prelude::Router;
use directories::ProjectDirs;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::router::Route;

pub static DIRS: Lazy<ProjectDirs> =
	Lazy::new(|| ProjectDirs::from("", "", "CofD.tools").expect("Project dirs"));

#[derive(Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Books(Vec<Book>);

impl Books {
	pub fn save(&self) -> anyhow::Result<()> {
		let data_path = DIRS.data_dir().join("books.json");

		serde_json::ser::to_writer_pretty(File::create(data_path)?, self)?;
		Ok(())
	}
}

impl Deref for Books {
	type Target = Vec<Book>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Books {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

// #[derive(Default, Serialize, Deserialize)]
// pub struct AppState {
// 	#[serde(skip, default)]
// 	pub books: Vec<Book>,
// }

#[derive(Default, Serialize, Deserialize)]
pub struct AppSettings {}

pub fn use_books(cx: &ScopeState) -> &UseSharedState<Books> {
	use_shared_state::<Books>(cx).expect("State not provided")
}

pub fn use_app_settings(cx: &ScopeState) -> &UseSharedState<AppSettings> {
	use_shared_state::<AppSettings>(cx).expect("Settings not provided")
}

fn read_books() -> anyhow::Result<Books> {
	let data_path: std::path::PathBuf = DIRS.data_dir().join("books.json");
	if !DIRS.data_dir().exists() {
		std::fs::create_dir_all(DIRS.data_dir())?;
		return Err(anyhow!(""));
	}

	if data_path.exists() && data_path.is_file() {
		Ok(Books(serde_json::de::from_reader(File::open(data_path)?)?))
	} else {
		Err(anyhow!(""))
	}
}

fn read_settings() -> anyhow::Result<AppSettings> {
	let config_path = DIRS.config_dir().join("settings.toml");
	if !DIRS.config_dir().exists() {
		std::fs::create_dir_all(DIRS.config_dir())?;
		return Err(anyhow!(""));
	}

	if config_path.exists() {
		Ok(toml::from_str(&std::fs::read_to_string(config_path)?)?)
	} else {
		Err(anyhow!(""))
	}
}

pub fn App(cx: Scope) -> Element {
	use_shared_state_provider(cx, || {
		read_books().unwrap_or_else(|err| {
			log::error!("{err}");
			Default::default()
		})
	});
	use_shared_state_provider(cx, || read_settings().unwrap_or_default());

	cx.render(rsx! {
		Router::<Route> {}
	})
}
