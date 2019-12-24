use plygui_api::{
    controls::{Member, Control},
    development::{AControl, ControlInner, HasInner, AMember},
};

define! {
    Scintilla: Control {
        outer: {
            fn set_margin_width(&mut self, index: usize, width: isize);
            fn set_readonly(&mut self, readonly: bool);
            fn is_readonly(&self) -> bool;
            /*fn set_codepage(&mut self, cp: Codepage); // if we manipulate UTF8 only, do we need this in public?
            fn codepage(&self) -> Codepage;*/
        
            fn append_text(&mut self, text: &str);
        }
        inner: {
            fn set_margin_width(&mut self, index: usize, width: isize);
            fn set_readonly(&mut self, readonly: bool);
            fn is_readonly(&self) -> bool;
            fn set_codepage(&mut self, cp: super::Codepage);
            fn codepage(&self) -> super::Codepage;
            fn append_text(&mut self, text: &str);
        }
        constructor: {
            fn new() -> Box<dyn Scintilla>;
        }
    }
}

impl<T: ScintillaInner> Scintilla for AMember<AControl<AScintilla<T>>> {
    fn set_margin_width(&mut self, index: usize, width: isize) {
        self.inner_mut().inner_mut().inner_mut().set_margin_width(index, width)
    }
    fn set_readonly(&mut self, readonly: bool) {
        self.inner_mut().inner_mut().inner_mut().set_readonly(readonly)
    }
    fn is_readonly(&self) -> bool {
        self.inner().inner().inner().is_readonly()
    }
    fn append_text(&mut self, text: &str) {
        self.inner_mut().inner_mut().inner_mut().append_text(text)
    }
    fn as_scintilla(& self) -> & dyn Scintilla { self } 
    fn as_scintilla_mut (& mut self) -> & mut dyn Scintilla { self } 
    fn into_scintilla (self : Box < Self >) -> Box < dyn Scintilla > { self }
}
impl<T: ScintillaInner> NewScintilla for AMember<AControl<AScintilla<T>>> {
    fn new() -> Box<dyn Scintilla> {
        T::new()
    }
}
