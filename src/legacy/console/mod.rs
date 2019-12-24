use super::development as scintilla_dev;
use super::*;

#[cfg(all(target_os = "macos", feature = "cocoa_"))]
pub(crate) mod lib_cocoa;
#[cfg(feature = "gtk3")]
pub(crate) mod lib_gtk;
#[cfg(feature = "qt5")]
pub(crate) mod lib_qt;
#[cfg(all(target_os = "windows", feature = "win32"))]
pub(crate) mod lib_win32;

pub(crate) mod common;
pub use self::common::Console;

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<dyn (::plygui_api::controls::Control)> {
    Console::new(false).into_control()
}
