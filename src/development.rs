pub use super::api::scintilla::{ScintillaInner, AScintilla};
pub use super::api::code_editor::{CodeEditorInner, ACodeEditor};
//pub use super::api::console::{ConsoleInner, AConsole};

#[cfg(all(target_os = "windows", feature = "win32"))]
pub use crate::imp::scintilla::mod_win32::WindowsScintillaInner as ScintillaControlInner;