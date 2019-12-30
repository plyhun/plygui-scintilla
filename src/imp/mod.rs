
pub mod scintilla;
pub mod code_editor;

#[cfg(all(target_os = "windows", feature = "win32"))]
pub use self::scintilla::mod_win32::Scintilla;