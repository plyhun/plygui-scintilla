use plygui_api::{
    controls::{Member},
    development::{AControl, HasInner, AMember},
};
use crate::Scintilla;
use crate::development::{ScintillaInner, AScintilla};

define! {
    CodeEditor: Scintilla {
        constructor: {
            fn with_content<S: AsRef<str>>(content: S) -> Box<dyn CodeEditor>;
        }
    }
}
impl<T: CodeEditorInner + Sized + 'static> ScintillaInner for ACodeEditor<T> {
    fn new() -> Box<dyn Scintilla> {
        <T as ScintillaInner>::new()
    }
    fn set_margin_width(&mut self, index: usize, width: isize) {
        self.inner_mut().set_margin_width(index, width)
    }
    fn set_readonly(&mut self, readonly: bool) {
        self.inner_mut().set_readonly(readonly)
    }
    fn is_readonly(&self) -> bool {
        self.inner().is_readonly()
    }
    fn set_codepage(&mut self, cp: super::Codepage) {
        self.inner_mut().set_codepage(cp)
    }
    fn codepage(&self) -> super::Codepage {
        self.inner().codepage()
    }
    fn append_text(&mut self, text: &str) {
        self.inner_mut().append_text(text)
    }
}
impl<T: CodeEditorInner + Sized + 'static> CodeEditor for AMember<AControl<AScintilla<ACodeEditor<T>>>> {
    fn as_code_editor(& self) -> & dyn CodeEditor { self } 
    fn as_code_editor_mut (& mut self) -> & mut dyn CodeEditor { self } 
    fn into_code_editor (self : Box < Self >) -> Box < dyn CodeEditor > { self }
}
impl<T: CodeEditorInner> NewCodeEditor for AMember<AControl<AScintilla<ACodeEditor<T>>>> {
    fn with_content<S: AsRef<str>>(content: S) -> Box<dyn CodeEditor> {
        T::with_content(content)
    }
}
