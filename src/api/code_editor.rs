use plygui_api::{
    controls::{Member, Control},
    development::{AControl, ControlInner, HasInner, AMember},
};

define! {
    CodeEditor: Control {
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
            fn with_content<S: AsRef<str>>(content: S) -> Box<dyn CodeEditor>;
        }
    }
}

impl<T: CodeEditorInner + Sized + 'static> CodeEditor for AMember<AControl<ACodeEditor<T>>> {
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
    fn as_code_editor(& self) -> & dyn CodeEditor { self } 
    fn as_code_editor_mut (& mut self) -> & mut dyn CodeEditor { self } 
    fn into_code_editor (self : Box < Self >) -> Box < dyn CodeEditor > { self }
}
impl<T: CodeEditorInner> NewCodeEditor for AMember<AControl<ACodeEditor<T>>> {
    fn with_content<S: AsRef<str>>(content: S) -> Box<dyn CodeEditor> {
        T::with_content(content)
    }
}
