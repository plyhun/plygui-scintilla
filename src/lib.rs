extern crate scintilla_sys;

#[macro_use]
extern crate lazy_static;
//#[macro_use]
extern crate plygui_api;

#[cfg(target_os="windows")]
mod lib_win32;
#[macro_use]
#[cfg(target_os="windows")]
extern crate plygui_win32;
#[cfg(target_os="windows")]
extern crate winapi;
#[cfg(target_os="windows")]
pub use lib_win32::Scintilla;

#[cfg(target_os="macos")]
mod lib_cocoa;
#[macro_use]
#[cfg(target_os="macos")]
extern crate plygui_cocoa;
#[cfg(target_os="macos")]
#[macro_use]
extern crate objc;
#[cfg(target_os="macos")]
extern crate cocoa;
#[cfg(target_os="macos")]
extern crate core_foundation;
#[cfg(target_os="macos")]
pub use lib_cocoa::Scintilla;

pub const MEMBER_ID_SCINTILLA: &str = "Scintilla";

pub trait UiScintilla: plygui_api::traits::UiControl {
    fn set_margin_width(&mut self, index: usize, width: isize);
}
