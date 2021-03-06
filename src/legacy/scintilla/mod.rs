use super::*;

#[cfg(all(target_os = "windows", feature = "win32"))]
pub(crate) mod lib_win32;
#[cfg(all(target_os = "windows", feature = "win32"))]
pub use self::lib_win32::Scintilla;

#[cfg(all(target_os = "macos", feature = "cocoa_"))]
pub(crate) mod lib_cocoa;
#[cfg(all(target_os = "macos", feature = "cocoa_"))]
pub use self::lib_cocoa::Scintilla;

#[cfg(feature = "qt5")]
pub(crate) mod lib_qt;
#[cfg(feature = "qt5")]
pub use self::lib_qt::Scintilla;

#[cfg(feature = "gtk3")]
pub(crate) mod lib_gtk;
#[cfg(feature = "gtk3")]
pub use self::lib_gtk::Scintilla;
