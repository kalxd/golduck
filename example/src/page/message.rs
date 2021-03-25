use gtk::prelude::*;
use gtk::Window;

use golduck::{builder::BuilderI, include_builder};

use crate::is_page::IsPage;

pub struct MessagePage {
	window: Window,
}

impl MessagePage {
	pub fn new() -> Self {
		let builder = include_builder!("../../ui/message.glade");
		let window = builder.get_by::<Window>("window");

		Self { window }
	}
}

impl IsPage for MessagePage {
	fn get_title(&self) -> &'static str {
		"信息提示"
	}

	fn show(&self) {
		self.window.show_all();
	}
}
