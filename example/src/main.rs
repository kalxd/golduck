use gtk::prelude::*;
use gtk::Application;

use golduck::{builder::BuilderI, gtk_app_main, include_builder};

use std::collections::HashMap;

mod is_page;
mod page;

use is_page::IsPage;

#[derive(PartialEq, Eq, Hash)]
enum PageType {
	MessageP,
}

fn gui_main(app: &Application) {
	let builder = include_builder!("../ui/main.glade");
	let window = builder.get_application_window_by("app");
	window.set_application(Some(app));

	let mut page_map: HashMap<PageType, Box<dyn IsPage>> = HashMap::new();

	page_map.insert(
		PageType::MessageP,
		Box::new(page::message::MessagePage::new()),
	);

	window.show_all();
}

gtk_app_main!("example.gtk.demo");
