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
use lib_win32 as inner_imp;

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
use lib_cocoa as inner_imp;

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
use lib_qt as inner_imp;

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
use lib_gtk as inner_imp;

pub trait Scintilla: plygui_api::controls::Control {
    fn set_margin_width(&mut self, index: usize, width: isize);
}

pub trait NewScintilla {
	fn new() -> Box<Scintilla>;
	fn with_content(content: &str) -> Box<Scintilla>;
}

pub mod imp {
	pub use inner_imp::Scintilla;
}
	
pub mod development {
	use plygui_api::development::*;
	
	pub trait ScintillaInner: ControlInner {
		fn new() -> Box<super::Scintilla>;
		fn with_content(content: &str) -> Box<super::Scintilla>;
		fn set_margin_width(&mut self, index: usize, width: isize);
	}
	
	impl <T: ScintillaInner + Sized + 'static> super::Scintilla for Member<Control<T>> {
		fn set_margin_width(&mut self, index: usize, width: isize) {
			self.as_inner_mut().as_inner_mut().set_margin_width(index, width)
		}
	}
	// still bloody but less fucking orphan rules
	impl <T: ScintillaInner + Sized> super::NewScintilla for Member<Control<T>> {
		fn new() -> Box<super::Scintilla> {
			T::new()
		}
		fn with_content(content: &str) -> Box<super::Scintilla> {
			T::with_content(content)
		}
	} 
}