//! [Builder][gtk::Builder]扩展模块。
use glib::object::{IsA, Object};
use gtk::prelude::BuilderExtManual;

// 内部使用宏，就是为了省点代码。
macro_rules! gen_get_by {
	($name:ident, $t:ty) => {
		fn $name(&self, name: &str) -> $t {
			self.get_by(name)
		}
	};
}

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

	gen_get_by!(get_application_window_by, gtk::ApplicationWindow);
	gen_get_by!(get_button_by, gtk::Button);
	gen_get_by!(get_entry, gtk::Entry);
	gen_get_by!(get_spin, gtk::SpinButton);
}

impl<W: IsA<gtk::Builder>> BuilderI for W {}

/// 将ui文件预编译进程序。
///
/// ```
/// let builder: Builder = builder!("./main.glade");
/// ```
#[macro_export]
macro_rules! builder {
	($path:literal) => {
		gtk::Builder::from_string(include_str!($path))
	};
}
