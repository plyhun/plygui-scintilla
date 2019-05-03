use scintilla_sys;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate plygui_api;

#[cfg(all(target_os = "macos", feature = "cocoa_"))]
#[macro_use]
extern crate objc;

#[cfg(feature = "qt5")]
extern crate plygui_qt;

#[cfg(feature = "gtk3")]
extern crate plygui_gtk;

mod console;
mod development;
mod scintilla;

pub trait Console: plygui_api::controls::Control + plygui_api::controls::HasLabel {
    fn exec(&mut self, command: &str);
}
pub trait NewConsole {
    fn new(with_command_line: bool) -> Box<dyn Console>;
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
    fn new() -> Box<dyn Scintilla>;
    fn with_content(content: &str) -> Box<dyn Scintilla>;
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
    pub use crate::console::Console;
    pub use crate::scintilla::Scintilla;
}
