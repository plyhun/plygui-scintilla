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
impl<T: CodeEditorInner + Sized + 'static> CodeEditor for AMember<AControl<AScintilla<ACodeEditor<T>>>> {
    default fn as_code_editor(& self) -> & dyn CodeEditor { self } 
    default fn as_code_editor_mut (& mut self) -> & mut dyn CodeEditor { self } 
    default fn into_code_editor (self : Box < Self >) -> Box < dyn CodeEditor > { self }
}
impl<T: CodeEditorInner> NewCodeEditor for AMember<AControl<AScintilla<ACodeEditor<T>>>> {
    fn with_content<S: AsRef<str>>(content: S) -> Box<dyn CodeEditor> {
        T::with_content(content)
    }
}
impl<II: CodeEditorInner, T: HasInner<I = II> + 'static> CodeEditorInner for T {
	default fn with_content<S: AsRef<str>>(content: S) -> Box<dyn CodeEditor> {
		<T::I as CodeEditorInner>::with_content(content)
	}
}
