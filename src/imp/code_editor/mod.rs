#[cfg(all(target_os = "windows", feature = "win32"))]
pub(crate) mod mod_win32;
#[cfg(all(target_os = "windows", feature = "win32"))]
pub use self::mod_win32::Scintilla;

#[cfg(all(target_os = "macos", feature = "cocoa_"))]
pub(crate) mod mod_cocoa;
#[cfg(all(target_os = "macos", feature = "cocoa_"))]
pub use self::mod_cocoa::Scintilla;

#[cfg(feature = "qt5")]
pub(crate) mod mod_qt;
#[cfg(feature = "qt5")]
pub use self::mod_qt::Scintilla;

#[cfg(feature = "gtk3")]
pub(crate) mod mod_gtk;
#[cfg(feature = "gtk3")]
pub use self::mod_gtk::Scintilla;