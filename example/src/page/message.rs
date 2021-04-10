use gtk::prelude::*;
use gtk::Window;

use golduck::{alert::AlertI, builder::BuilderI, include_builder};

use super::IsPage;

#[derive(Clone)]
pub struct MessagePage {
	window: Window,
}

pub fn new() -> MessagePage {
	let builder = include_builder!("../../ui/message.glade");
	let window = builder.get_by::<Window>("window");

	let page = MessagePage { window };

	{
		let page = page.clone();
		builder
			.get_button_by("normal_btn")
			.connect_clicked(move |_| {
				page.show_alert("这是一个提示。");
			});
	}

	{
		let page = page.clone();
		builder
			.get_button_by("error_btn")
			.connect_clicked(move |_| {
				page.show_error("你看到了一个错误提示！");
			});
	}

	{
		let page = page.clone();
		builder.get_button_by("warn_btn").connect_clicked(move |_| {
			page.show_warn("你看了一个警告。");
		});
	}

	return page;
}

impl AlertI for MessagePage {
	type Parent = Window;

	fn get_parent(&self) -> Option<&Self::Parent> {
		Some(&self.window)
	}
}

impl IsPage for MessagePage {
	fn show(&self) {
		self.window.show_all();
	}
}
