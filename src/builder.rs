//! [Builder][gtk::Builder]扩展模块。
use glib::object::{IsA, Object};
use gtk::prelude::BuilderExtManual;

pub trait BuilderI: BuilderExtManual {
	/// [gtk::prelude::BuilderExtManual::get_object]简化版，一般而言，我们很少关心`name`不存在情况。
	/// 如果不存在，直接退出，并尽可能留下有用信息。
	///
	/// # 崩溃！
	/// `name`不存在导致运行时崩溃。
	fn get_by<W: IsA<Object>>(&self, name: &str) -> W {
		match self.get_object(name) {
			Some(w) => w,
			None => {
				let msg = format!("{:?} not found!", name);
				panic!(msg);
			}
		}
	}
}
