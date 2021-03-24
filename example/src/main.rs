use gtk::prelude::*;
use gtk::Application;

use golduck::{builder, builder::BuilderI, gtk_app_main};

fn app_run(app: &Application) {
	let builder = builder!("../ui/main.glade");

	let window = builder.get_application_window_by("app");
	window.set_application(Some(app));
	window.show_all();
}

gtk_app_main!("example.gtk.demo", app_run);
