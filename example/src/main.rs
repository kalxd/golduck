use gtk::prelude::*;
use gtk::{Application, Stack};

use std::collections::HashMap;
use std::rc::Rc;

use golduck::{builder::BuilderI, gtk_app_main, include_builder};

mod page;

use page::IsPage;

#[derive(Hash, PartialEq, Eq)]
enum ExampleType {
	MessageT,
}

impl From<i32> for ExampleType {
	fn from(i: i32) -> Self {
		match i {
			_ => Self::MessageT,
		}
	}
}

type Callback = Box<dyn Fn() -> Box<dyn IsPage>>;

fn callback<F>(f: F) -> Callback
where
	F: Fn() -> Box<dyn IsPage> + 'static,
{
	Box::new(f) as Callback
}

fn gui_main(app: &Application) {
	let builder = include_builder!("../ui/main.glade");
	let window = builder.get_application_window_by("app");
	window.set_application(Some(app));

	let example_map = {
		let mut example_map: HashMap<ExampleType, Callback> = HashMap::new();
		example_map.insert(
			ExampleType::MessageT,
			callback(|| Box::new(page::message::new())),
		);
		Rc::new(example_map)
	};

	{
		let button = builder.get_button_by("run_btn");
		let stack = builder.get_by::<Stack>("stack1");
		button.connect_clicked(move |_| {
			let widget = stack
				.get_visible_child()
				.map(|child| stack.get_child_position(&child))
				.and_then(|i| {
					let t = i.into();
					example_map.get(&t)
				});
			if let Some(f) = widget {
				f().show();
			}
		});
	}

	window.show_all();
}

gtk_app_main!("example.gtk.demo");
