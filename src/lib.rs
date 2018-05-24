extern crate scintilla_sys;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate plygui_api;

#[cfg(all(target_os = "windows", feature = "win32"))]
mod lib_win32;
#[cfg(all(target_os = "windows", feature = "win32"))]
extern crate plygui_win32;
#[cfg(all(target_os = "windows", feature = "win32"))]
extern crate winapi;
#[cfg(all(target_os = "windows", feature = "win32"))]
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

#[cfg(feature = "qt5")]
mod lib_qt;
#[macro_use]
#[cfg(feature = "qt5")]
extern crate plygui_qt;
#[cfg(feature = "qt5")]
extern crate qt_core;
#[cfg(feature = "qt5")]
extern crate qt_widgets;
#[cfg(feature = "qt5")]
extern crate qt_gui;
#[cfg(feature = "qt5")]
pub use lib_qt::Scintilla;

#[cfg(feature = "gtk3")]
mod lib_gtk;
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
#[cfg(feature = "gtk3")]
pub use lib_gtk::Scintilla;

pub trait UiScintilla: plygui_api::traits::UiControl {
    fn set_margin_width(&mut self, index: usize, width: isize);
}

pub trait NewScintilla {
	fn new() -> Box<UiScintilla>;
	fn with_content(content: &str) -> Box<UiScintilla>;
}
	
pub mod development {
	use super::*;
	use plygui_api::development::*;
	
	pub trait ScintillaInner: ControlInner {
		fn new() -> Box<UiScintilla>;
		fn with_content(content: &str) -> Box<UiScintilla>;
		fn set_margin_width(&mut self, index: usize, width: isize);
	}
	
	impl <T: ScintillaInner + Sized + 'static> UiScintilla for Member<Control<T>> {
		fn set_margin_width(&mut self, index: usize, width: isize) {
			self.as_inner_mut().as_inner_mut().set_margin_width(index, width)
		}
	}
	// still bloody but less fucking orphan rules
	impl <T: ScintillaInner + Sized> NewScintilla for Member<Control<T>> {
		fn new() -> Box<UiScintilla> {
			T::new()
		}
		fn with_content(content: &str) -> Box<UiScintilla> {
			T::with_content(content)
		}
	} 
}