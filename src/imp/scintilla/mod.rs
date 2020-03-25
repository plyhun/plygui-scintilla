#[cfg(all(target_os = "windows", feature = "win32"))]
pub(crate) mod mod_win32;
#[cfg(all(target_os = "windows", feature = "win32"))]
pub use mod_win32::{Scintilla, WindowsScintilla as ScintillaControl};


#[cfg(all(target_os = "macos", feature = "cocoa_"))]
pub(crate) mod mod_cocoa;
#[cfg(all(target_os = "macos", feature = "cocoa_"))]
pub use mod_cocoa::{Scintilla, CocoaScintilla as ScintillaControl};

#[cfg(feature = "qt5")]
pub(crate) mod mod_qt;
#[cfg(feature = "qt5")]
pub use mod_qt::{Scintilla, QtScintilla as ScintillaControl};


#[cfg(feature = "gtk3")]
pub(crate) mod mod_gtk;
#[cfg(feature = "gtk3")]
pub use mod_gtk::{Scintilla, GtkScintilla as ScintillaControl};
