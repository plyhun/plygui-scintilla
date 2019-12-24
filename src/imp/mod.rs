
pub mod scintilla;

#[cfg(all(target_os = "windows", feature = "win32"))]
pub use self::scintilla::mod_win32::Scintilla;