//! 弹窗相关功能实现。
use glib::object::IsA;
use gtk::prelude::DialogExt;
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType, Window};

// 内部宏。
macro_rules! gen_by {
	($name:ident, $msg_type:expr) => {
		fn $name<S: AsRef<str>>(&self, msg: S) {
			self.show_message($msg_type, msg)
		}
	};
}

/// 一次性弹窗功能，仅用于显示一次性信息。
pub trait AlertI {
	type Parent: IsA<Window>;

	fn get_parent(&self) -> Option<&Self::Parent>;

	fn show_message<S>(&self, msg_type: MessageType, msg: S)
	where
		S: AsRef<str>,
	{
		let parent: Option<&Self::Parent> = self.get_parent();

		let dialog = MessageDialog::new(
			parent,
			DialogFlags::empty(),
			msg_type,
			ButtonsType::Ok,
			msg.as_ref(),
		);

		dialog.run();
		dialog.emit_close();
	}

	gen_by!(show_alert, MessageType::Info);
	gen_by!(show_error, MessageType::Error);
	gen_by!(show_warn, MessageType::Warning);
}
