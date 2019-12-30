#![feature(specialization)]

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

#[cfg(feature = "qt5")]
extern crate plygui_qt;

#[cfg(feature = "gtk3")]
extern crate plygui_gtk;

mod development;

pub mod imp;
pub mod api;

//pub use crate::api::console::{Console, NewConsole};
pub use crate::api::code_editor::{CodeEditor, NewCodeEditor};
pub use crate::api::scintilla::{Scintilla, NewScintilla};
pub use crate::api::Codepage;
