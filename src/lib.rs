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
#[cfg(all(target_os = "macos", feature = "cocoa_"))]
extern crate plygui_cocoa;
#[cfg(all(target_os = "macos", feature = "cocoa_"))]
#[macro_use]
extern crate objc;
#[cfg(all(target_os = "macos", feature = "cocoa_"))]
extern crate cocoa;
#[cfg(all(target_os = "macos", feature = "cocoa_"))]
extern crate core_foundation;

#[macro_use]
#[cfg(feature = "qt5")]
extern crate plygui_qt;
#[cfg(feature = "qt5")]
extern crate qt_core;
#[cfg(feature = "qt5")]
extern crate qt_gui;
#[cfg(feature = "qt5")]
extern crate qt_widgets;

#[macro_use]
#[cfg(feature = "gtk3")]
extern crate plygui_gtk;
#[cfg(feature = "gtk3")]
extern crate gdk;
#[cfg(feature = "gtk3")]
extern crate glib;
#[cfg(feature = "gtk3")]
extern crate gtk;
#[cfg(feature = "gtk3")]
extern crate pango;

mod console;
mod development;
mod scintilla;

pub trait Console: plygui_api::controls::Control + plygui_api::controls::HasLabel {
    fn exec(&mut self, command: &str);
}
pub trait NewConsole {
    fn new(with_command_line: bool) -> Box<Console>;
}

pub trait Scintilla: plygui_api::controls::Control {
    fn set_margin_width(&mut self, index: usize, width: isize);
    fn set_readonly(&mut self, readonly: bool);
    fn is_readonly(&self) -> bool;
    /*fn set_codepage(&mut self, cp: Codepage); // if we manipulate UTF8 only, do we need this in public?
    fn codepage(&self) -> Codepage;*/

    fn append_text(&mut self, text: &str);
}
pub trait NewScintilla {
    fn new() -> Box<Scintilla>;
    fn with_content(content: &str) -> Box<Scintilla>;
}

pub enum Codepage {
    Ascii = 0isize,
    Utf8 = 65001isize,
    ShiftJis = 932isize,
    ChineseSimplifiedGbk = 936isize,
    KoreanUnifiedHangul = 949isize,
    ChineseTraditionalBig5 = 950isize,
    KoreanJohab = 1361isize,
}
impl From<isize> for Codepage {
    fn from(i: isize) -> Self {
        match i {
            0isize => Codepage::Ascii,
            65001isize => Codepage::Utf8,
            932isize => Codepage::ShiftJis,
            936isize => Codepage::ChineseSimplifiedGbk,
            949isize => Codepage::KoreanUnifiedHangul,
            950isize => Codepage::ChineseTraditionalBig5,
            1361isize => Codepage::KoreanJohab,
            _ => panic!("Unsupported codepage id: {}", i),
        }
    }
}
impl Default for Codepage {
    fn default() -> Self {
        Codepage::Utf8
    }
}

pub mod imp {
    pub use console::Console;
    pub use scintilla::Scintilla;
}
