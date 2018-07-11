extern crate scintilla_sys;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate plygui_api;

#[cfg(all(target_os = "windows", feature = "win32"))]
extern crate plygui_win32;
#[cfg(all(target_os = "windows", feature = "win32"))]
extern crate winapi;

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

#[macro_use]
#[cfg(feature = "qt5")]
extern crate plygui_qt;
#[cfg(feature = "qt5")]
extern crate qt_core;
#[cfg(feature = "qt5")]
extern crate qt_widgets;
#[cfg(feature = "qt5")]
extern crate qt_gui;

#[macro_use]
#[cfg(feature = "gtk3")]
extern crate plygui_gtk;
#[cfg(feature = "gtk3")]
extern crate gtk;
#[cfg(feature = "gtk3")]
extern crate gdk;
#[cfg(feature = "gtk3")]
extern crate glib;
#[cfg(feature = "gtk3")]
extern crate pango;

mod development;
mod scintilla;
mod console;

pub trait Console: plygui_api::controls::Control + plygui_api::controls::HasLabel {
	fn exec(&mut self, command: &str);
}
pub trait NewConsole {
	fn new(with_command_line: bool) -> Box<Console>;
}

pub trait Scintilla: plygui_api::controls::Control {
    fn set_margin_width(&mut self, index: usize, width: isize);
}
pub trait NewScintilla {
	fn new() -> Box<Scintilla>;
	fn with_content(content: &str) -> Box<Scintilla>;
}

pub mod imp {
	pub use scintilla::Scintilla;
	pub use console::Console;
}
