pub trait IsPage {
	fn get_title(&self) -> &'static str;

	fn show(&self);
}
