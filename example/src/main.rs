use gtk::prelude::*;
use gtk::{Application, Window};

use golduck::gtk_app_main;

fn app_run(app: &Application) {
	let window = Window::new(gtk::WindowType::Toplevel);
	window.set_application(Some(app));
	window.show_all();
}

gtk_app_main!("example.gtk.demo", app_run);
