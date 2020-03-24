pub use super::api::scintilla::{ScintillaInner, AScintilla, NewScintillaInner};
pub use super::api::code_editor::{CodeEditorInner, ACodeEditor, NewCodeEditorInner};
pub use super::api::console::{ConsoleInner, AConsole, NewConsoleInner};

#[cfg(all(target_os = "windows", feature = "win32"))]
pub use crate::imp::scintilla::ScintillaControl;

#[cfg(feature = "qt5")]
pub use crate::imp::scintilla::ScintillaControl;