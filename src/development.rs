#[cfg(all(target_os = "windows", feature = "win32"))]
use std::sync::atomic::AtomicUsize;
#[cfg(all(target_os = "windows", feature = "win32"))]
pub(crate) static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

pub use super::api::scintilla::{ScintillaInner, AScintilla};
pub use super::api::code_editor::{CodeEditorInner, ACodeEditor};
//pub use super::api::console::{ConsoleInner, AConsole};

#[cfg(all(target_os = "windows", feature = "win32"))]
pub use super::imp::scintilla::mod_win32::WindowsScintillaInner as ScintillaControlInner;