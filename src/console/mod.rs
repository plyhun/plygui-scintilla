use super::*;

#[cfg(all(target_os = "windows", feature = "win32"))]
pub(crate) mod lib_win32;
#[cfg(all(target_os = "windows", feature = "win32"))]
pub use self::lib_win32::Console;

#[cfg(target_os="macos")]
pub(crate) mod lib_cocoa;
#[cfg(target_os="macos")]
pub use self::lib_cocoa::Console;

#[cfg(feature = "qt5")]
pub(crate) mod lib_qt;
#[cfg(feature = "qt5")]
pub use self::lib_qt::Console;

#[cfg(feature = "gtk3")]
pub(crate) mod lib_gtk;
#[cfg(feature = "gtk3")]
pub use self::lib_gtk::Console;
