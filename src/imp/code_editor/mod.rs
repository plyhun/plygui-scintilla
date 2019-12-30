use crate::development::*;
use plygui_api::development::*;

pub type CodeEditor = AMember<AControl<AScintilla<ScintillaCodeEditor>>>;

pub struct ScintillaCodeEditor {
    inner: ScintillaControlInner<CodeEditor>
}
impl HasInner for ScintillaCodeEditor {
    type I = ScintillaControlInner<CodeEditor>;
    
    fn inner(&self) -> &Self::I { &self.inner }
    fn inner_mut(&mut self) -> &mut Self::I { &mut self.inner }
    fn into_inner(self) -> Self::I { self.inner }
}
impl ScintillaInner for ScintillaCodeEditor {
    fn new() -> Box<dyn crate::Scintilla> {
    	let b: Box<CodeEditor> = Box::new(AMember::with_inner(
            AControl::with_inner(
                AScintilla::with_inner(
                    CodeEditor {
                        inner: ScintillaControlInner::new_inner(),
                    }
                )
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        b
       // <ScintillaControlInner<CodeEditor> as ScintillaInner>::new()
    }
    fn set_margin_width(&mut self, index: usize, width: isize) {
        self.inner.set_margin_width(index, width)
    }
    fn set_readonly(&mut self, readonly: bool) {
        self.inner.set_readonly(readonly)
    }
    fn is_readonly(&self) -> bool {
        self.inner.is_readonly()
    }
    fn set_codepage(&mut self, cp: crate::Codepage) {
        self.inner.set_codepage(cp)
    }
    fn codepage(&self) -> crate::Codepage {
        self.inner.codepage()
    }
    fn append_text(&mut self, text: &str) {
        self.inner.append_text(text)
    }
}
impl CodeEditorInner for ScintillaCodeEditor {
	fn with_content<S: AsRef<str>>(content: S) -> Box<dyn crate::CodeEditor> {
		use crate::Scintilla;
		
		let mut b = Self::new();
		b.append_text(content.as_ref());
		b.into_any().downcast::<CodeEditor>().into_code_editor()
	}
}
default_impls_as!(CodeEditor);
