use plygui_api::{
    controls::{Member},
    sdk::{AControl, HasInner, AMember, Abstract},
};
use crate::{Scintilla};
use crate::sdk::{ScintillaInner, AScintilla};

define! {
    CodeEditor: Scintilla {
        constructor: {
            fn with_content<S: AsRef<str>>(content: S) -> Box<dyn CodeEditor>;
        }
    }
}
impl<T: CodeEditorInner + Sized + 'static> CodeEditor for AMember<AControl<AScintilla<ACodeEditor<T>>>> {
    default fn as_code_editor(& self) -> & dyn CodeEditor { self } 
    default fn as_code_editor_mut (& mut self) -> & mut dyn CodeEditor { self } 
    default fn into_code_editor (self : Box < Self >) -> Box < dyn CodeEditor > { self }
}
impl<T: CodeEditorInner> ScintillaInner for ACodeEditor<T> {
    default fn new() -> Box<dyn Scintilla> {
        <<Self as HasInner>::I as ScintillaInner>::new()
    }
    default fn set_margin_width(&mut self, index: usize, width: isize) {
        self.inner_mut().set_margin_width(index, width)
    }
    default fn set_readonly(&mut self, readonly: bool) {
        self.inner_mut().set_readonly(readonly)
    }
    default fn is_readonly(&self) -> bool {
        self.inner().is_readonly()
    }
    default fn set_codepage(&mut self, cp: super::Codepage) {
        self.inner_mut().set_codepage(cp)
    }
    default fn codepage(&self) -> super::Codepage {
        self.inner().codepage()
    }
    default fn append_text(&mut self, text: &str) {
        self.inner_mut().append_text(text)
    }
}

impl<T: CodeEditorInner> NewCodeEditor for AMember<AControl<AScintilla<ACodeEditor<T>>>> {
    fn with_content<S: AsRef<str>>(content: S) -> Box<dyn CodeEditor> {
        T::with_content(content)
    }
}
impl<II: CodeEditorInner, T: HasInner<I = II> + Abstract + 'static> CodeEditorInner for T {
	default fn with_content<S: AsRef<str>>(content: S) -> Box<dyn CodeEditor> {
		<T::I as CodeEditorInner>::with_content(content)
	}
}
