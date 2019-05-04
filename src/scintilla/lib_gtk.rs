//use super::development as scintilla_dev;

use plygui_gtk::common::*;

use scintilla_sys::{self, Ptr, /*SCNotification,*/ Scintilla as GtkScintilla, ScintillaExt};

use std::str;

pub type Scintilla = Member<Control<ScintillaGtk>>;

#[repr(C)]
pub struct ScintillaGtk {
    base: GtkControlBase<Scintilla>,
}

impl super::development::ScintillaInner for ScintillaGtk {
    fn new() -> Box<super::Scintilla> {
        let sc = GtkScintilla::new();
        let mut sc = Box::new(Member::with_inner(
            Control::with_inner(
                ScintillaGtk {
                    base: GtkControlBase::with_gtk_widget(sc.upcast::<Widget>()),
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));

        {
            let ptr = sc.as_ref() as *const _ as *mut c_void;
            sc.as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        {
            let sc: Object = Object::from(sc.as_inner_mut().as_inner_mut().base.widget.clone()).into();
            let sc = sc.downcast::<GtkScintilla>().unwrap();
            sc.connect_notify(on_notify);
        }
        Object::from(sc.as_inner_mut().as_inner_mut().base.widget.clone()).downcast::<Widget>().unwrap().connect_size_allocate(on_size_allocate);
        sc
    }
    fn set_margin_width(&mut self, index: usize, width: isize) {
        let widget: Object = Object::from(self.base.widget.clone()).into();
        widget.downcast::<GtkScintilla>().unwrap().send_message(scintilla_sys::SCI_SETMARGINWIDTHN as u32, index as u64, width as i64);
    }
    fn set_readonly(&mut self, readonly: bool) {
        let widget: Object = Object::from(self.base.widget.clone()).into();
        widget.downcast::<GtkScintilla>().unwrap().send_message(scintilla_sys::SCI_SETREADONLY as u32, if readonly { 1 } else { 0 }, 0);
    }
    fn is_readonly(&self) -> bool {
        let widget: Object = Object::from(self.base.widget.clone()).into();
        widget.downcast::<GtkScintilla>().unwrap().send_message(scintilla_sys::SCI_GETREADONLY as u32, 0, 0) == 1
    }
    fn set_codepage(&mut self, cp: super::Codepage) {
        let widget: Object = Object::from(self.base.widget.clone()).into();
        widget.downcast::<GtkScintilla>().unwrap().send_message(scintilla_sys::SCI_SETCODEPAGE as u32, cp as isize as u64, 0);
    }
    fn codepage(&self) -> super::Codepage {
        let widget: Object = Object::from(self.base.widget.clone()).into();
        (widget.downcast::<GtkScintilla>().unwrap().send_message(scintilla_sys::SCI_GETCODEPAGE as u32, 0, 0) as isize).into()
    }
    fn append_text(&mut self, text: &str) {
        self.set_codepage(super::Codepage::Utf8);
        let len = text.len();
        let tptr = text.as_bytes().as_ptr();
        let widget: Object = Object::from(self.base.widget.clone()).into();
        widget.downcast::<GtkScintilla>().unwrap().send_message(scintilla_sys::SCI_APPENDTEXT as u32, len as u64, tptr as i64);
    }
}

impl HasLayoutInner for ScintillaGtk {
    fn on_layout_changed(&mut self, _: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for ScintillaGtk {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.measure(member, control, pw, ph);
        control.coords = Some((x, y));
        self.draw(member, control);
    }
    fn on_removed_from_container(&mut self, _: &mut MemberBase, _: &mut ControlBase, _: &controls::Container) {}

    fn parent(&self) -> Option<&controls::Member> {
        self.base.parent().map(|m| m.as_member())
    }
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.parent_mut().map(|m| m.as_member_mut())
    }
    fn root(&self) -> Option<&controls::Member> {
        self.base.root().map(|m| m.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.root_mut().map(|m| m.as_member_mut())
    }
}


impl HasNativeIdInner for ScintillaGtk {
    type Id = GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl HasSizeInner for ScintillaGtk {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        self.base.widget().set_size_request(width as i32, height as i32);
        true
    }
}

impl HasVisibilityInner for ScintillaGtk {
    fn on_visibility_set(&mut self, _: &mut MemberBase, _: types::Visibility) -> bool {
        self.base.invalidate()
    }
}

impl MemberInner for ScintillaGtk {}

impl Drawable for ScintillaGtk {
    fn draw(&mut self, _: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(control);
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        let (lm, tm, rm, bm) = self.base.margins().into();

        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => w,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => {
                        42 as u16 // TODO min_width
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => h,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => {
                        42 as u16 // TODO min_height
                    }
                };
                (cmp::max(0, w as i32 + lm + rm) as u16, cmp::max(0, h as i32 + tm + bm) as u16)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    fn invalidate(&mut self, _: &mut MemberBase, _: &mut ControlBase) {
        self.base.invalidate();
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
    use crate::NewScintilla;
    
    Scintilla::new().into_control()
}

fn on_size_allocate(this: &::plygui_gtk::gtk::Widget, _allo: &::plygui_gtk::gtk::Rectangle) {
    use plygui_api::controls::HasSize;
    
    let mut ll = this.clone().upcast::<Widget>();
    let ll = cast_gtk_widget_to_member_mut::<Scintilla>(&mut ll).unwrap();

    let measured_size = ll.size();
    ll.call_on_size(measured_size.0 as u16, measured_size.1 as u16);
}

fn on_notify(_this: &GtkScintilla, _msg: i32, _notification: Ptr, _data: Ptr) {
    //let mut b = this.clone().upcast::<Widget>();
    //let notification = unsafe { &*(notification as *const SCNotification) };
}

default_impls_as!(Scintilla);
