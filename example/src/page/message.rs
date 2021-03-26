use gtk::prelude::*;
use gtk::Window;

use golduck::{builder::BuilderI, include_builder};

use super::IsPage;

pub struct MessagePage {
	window: Window,
}

pub fn new() -> MessagePage {
	let builder = include_builder!("../../ui/message.glade");
	let window = builder.get_by::<Window>("window");

	MessagePage { window }
}

impl IsPage for MessagePage {
	fn show(&self) {
		self.window.show_all();
	}
}
