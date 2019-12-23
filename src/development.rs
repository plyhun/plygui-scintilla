#[cfg(all(target_os = "windows", feature = "win32"))]
use std::sync::atomic::AtomicUsize;
#[cfg(all(target_os = "windows", feature = "win32"))]
pub(crate) static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

pub use super::api::code_editor::{CodeEditorInner, ACodeEditor};
pub use super::api::console::{ConsoleInner, AConsole};