use crate::development::*;
use plygui_api::*;
use plygui_api::development::*;

pub type CodeEditor = AMember<AControl<AScintilla<ACodeEditor<ScintillaCodeEditor>>>>;

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
        Self::with_content("").into_scintilla()	
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
	    let mut b: Box<CodeEditor> = Box::new(AMember::with_inner(
            AControl::with_inner(
                AScintilla::with_inner(
                    ACodeEditor::with_inner(
                        ScintillaCodeEditor {
                            inner: ScintillaControlInner::new_inner(),
                        }
                    )
                )
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        b.append_text(content.as_ref());
		b
	}
}
impl ControlInner for ScintillaCodeEditor {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.inner.on_added_to_container(member, control, parent, x, y, pw, ph)
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, c: &dyn controls::Container) {
        self.inner.on_removed_from_container(member, control, c)
    }

    fn parent(&self) -> Option<&dyn controls::Member> {
        self.inner.parent()
    }
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.parent_mut()
    }
    fn root(&self) -> Option<&dyn controls::Member> {
        self.inner.root()
    }
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.root_mut()
    }
}
impl HasVisibilityInner for ScintillaCodeEditor {
    fn on_visibility_set(&mut self, base: &mut MemberBase, visibility: types::Visibility) -> bool {
        self.inner.on_visibility_set(base, visibility)
    }
}
impl HasNativeIdInner for ScintillaCodeEditor {
    type Id = <ScintillaControlInner<CodeEditor> as HasNativeIdInner>::Id;

    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}
impl HasSizeInner for ScintillaCodeEditor {
    fn on_size_set(&mut self, base: &mut MemberBase, size: (u16, u16)) -> bool {
        self.inner.on_size_set(base, size)
    }
}
impl HasLayoutInner for ScintillaCodeEditor {
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base)
    }
}
impl MemberInner for ScintillaCodeEditor {}
impl Spawnable for ScintillaCodeEditor {
    fn spawn() -> Box<dyn controls::Control> {
        Self::new().into_control()
    }
}
impl Drawable for ScintillaCodeEditor {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.draw(member, control)
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        self.inner.measure(member, control, w, h)
    }
    fn invalidate(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.invalidate(member, control)
    }
}
impl crate::Scintilla for CodeEditor {
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
    fn as_scintilla(& self) -> & dyn crate::Scintilla { self } 
    fn as_scintilla_mut (& mut self) -> & mut dyn crate::Scintilla { self } 
    fn into_scintilla (self : Box < Self >) -> Box < dyn crate::Scintilla > { self }
}
default_impls_as!(CodeEditor);
