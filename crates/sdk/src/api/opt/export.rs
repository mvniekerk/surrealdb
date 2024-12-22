use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
#[non_exhaustive]
#[cfg_attr(docsrs, doc(cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"))))))]
pub enum ExportDestination {
	File(PathBuf),
	Memory,
}

/// A trait for converting inputs into database export locations
#[cfg_attr(docsrs, doc(cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"))))))]
pub trait IntoExportDestination<R> {
	/// Converts an input into a database export location
	fn into_export_destination(self) -> R;
}

impl<T> IntoExportDestination<PathBuf> for T
where
	T: AsRef<Path>,
{
	fn into_export_destination(self) -> PathBuf {
		self.as_ref().to_path_buf()
	}
}

impl IntoExportDestination<()> for () {
	fn into_export_destination(self) {}
}
