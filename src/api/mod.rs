pub mod scintilla;
pub mod code_editor;
//pub mod console;

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
