use gtk::prelude::*;
use gtk::Application;

use golduck::{builder::BuilderI, gtk_app_main, include_builder};

fn gui_main(app: &Application) {
	let builder = include_builder!("../ui/main.glade");

	let window = builder.get_application_window_by("app");
	window.set_application(Some(app));
	window.show_all();
}

gtk_app_main!("example.gtk.demo");
