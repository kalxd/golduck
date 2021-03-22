use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::Application;

pub fn gtk_app_run<F>(id: &str, f: F)
where
	F: Fn(&Application) + 'static,
{
	let app = Application::new(Some(id), gio::ApplicationFlags::FLAGS_NONE)
		.expect("gtk init failed within goduck lib!");

	app.connect_activate(f);

	app.run(&[]);
}
