//! gtk3工具类库。
//!
//! 该库并没有统一设计方向，目的仅仅为了简便一些常用代码，提供简单接口方便调用。
//! 需要使用功能，需要到对应模块下寻找。
use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::Application;

/// 简化gtk初始函数，仅提供一个`application_id`和初始成功后的回调即可。
///
/// ```
/// // 这里就是成功回调，一切初始化都在这里。
/// fn gui_main(app: &Application){
/// 	let window = Window::new(WindowType::Toplevel);
/// 	window.set_application(Some(app));
/// 	window.show_all();
/// }
///
/// fn main() {
/// 	gtk_app_run("my.application.example", gui_main);
/// }
/// ```
pub fn gtk_app_run<F>(id: &str, f: F)
where
	F: Fn(&Application) + 'static,
{
	let app = Application::new(Some(id), gio::ApplicationFlags::FLAGS_NONE)
		.expect("gtk init failed within goduck lib!");

	app.connect_activate(f);

	app.run(&[]);
}

/// 自动生成`main`函数，并在其中调用[gtk_app_run]。
///
/// ```
/// gtk_app_main!("id", gui_main);
/// ```
///
/// ```
/// fn main() {
/// 	gtk_app_run("id", gui_main);
/// }
/// ```
/// 以上两都写法等价。
#[macro_export]
macro_rules! gtk_app_main {
	($id:literal, $f:ident) => {
		fn main() {
			golduck::gtk_app_run($id, $f);
		}
	};
}
