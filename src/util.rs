//! 一些辅助工具。在某些场合下有些许帮助。

/// 静默选项，忽略不想处理的数据。
pub trait Silence: Sized {
	fn silent(self) {}
}

impl<T> Silence for Option<T> {}
impl<E, T> Silence for Result<E, T> {}
