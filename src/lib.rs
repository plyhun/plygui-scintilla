#![feature(specialization)]
#![feature(new_uninit)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate plygui_api;
#[macro_use]
extern crate plygui_macros;

extern crate scintilla_sys;

#[cfg(all(target_os = "macos", feature = "cocoa_"))]
#[macro_use]
extern crate objc;

#[cfg(all(target_os = "macos", feature = "cocoa_"))]
extern crate plygui_cocoa;

#[cfg(all(target_os = "windows", feature = "win32"))]
extern crate plygui_win32;

#[cfg(feature = "qt5")]
extern crate plygui_qt;

#[cfg(feature = "gtk3")]
extern crate plygui_gtk;

pub mod sdk;

pub mod imp;
pub mod api;

pub use crate::api::console::{Console, NewConsole};
pub use crate::api::code_editor::{CodeEditor, NewCodeEditor};
pub use crate::api::scintilla::{Scintilla, NewScintilla};
pub use crate::api::Codepage;
