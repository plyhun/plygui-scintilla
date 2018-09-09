use super::development as scintilla_dev;

use plygui_qt::common::*;
use scintilla_sys::*;
use std::os::raw::{c_int, c_uint};

pub type Scintilla = Member<Control<ScintillaQt>>;

#[repr(C)]
pub struct ScintillaQt {
    base: QtControlBase<Scintilla, ScintillaEditBase>,

    h_command: (bool, SlotSCNotificationPtr<'static>),
    ui_cb: Option<scintilla_dev::Custom>,
    fn_ptr: Option<extern "C" fn(*mut c_void, c_int, c_int, c_int)>,
    self_ptr: Option<*mut c_void>,
}

impl scintilla_dev::ScintillaInner for ScintillaQt {
    fn set_margin_width(&mut self, index: usize, width: isize) {
        unsafe { let _ = self.base.widget.as_mut().send(SCI_SETMARGINWIDTHN as u32, index as c_uint, width as c_int); }
    }
    fn new() -> Box<super::Scintilla> {
        let mut sc = ScintillaEditBase::new();
        let (fn_ptr, self_ptr) = unsafe {
            let self_ptr = sc.as_mut().send(SCI_GETDIRECTPOINTER, 0, 0);
            let fn_ptr = sc.as_mut().send(SCI_GETDIRECTFUNCTION, 0, 0);
            (fn_ptr, self_ptr)
        };
        let mut sc = Box::new(Member::with_inner(
            Control::with_inner(
                ScintillaQt {
                    base: QtControlBase::with_params(sc, event_handler),
                    ui_cb: None,
                    h_command: (false, SlotSCNotificationPtr::new(move |_| {})),
                    fn_ptr: Some(unsafe { mem::transmute(fn_ptr) }),
                    self_ptr: Some(self_ptr as *mut c_void),
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        unsafe {
            use qt_core::cpp_utils::StaticCast;
            let ptr = sc.as_ref() as *const _ as u64;
            let qo: &mut QObject = sc.as_inner_mut().as_inner_mut().base.widget.static_cast_mut();
            qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        sc.as_inner().as_inner().base.widget.signals().notify().connect(&sc.as_inner().as_inner().h_command.1);
        sc
    }
    fn set_readonly(&mut self, readonly: bool) {
        unsafe { let _ = self.base.widget.as_mut().send(SCI_SETREADONLY as u32, if readonly { 1 } else { 0 }, 0); }
    }
    fn is_readonly(&self) -> bool {
        unsafe { self.base.widget.as_ref().send(SCI_GETREADONLY, 0, 0) as usize == 1 }
    }
    fn set_codepage(&mut self, cp: super::Codepage) {
        unsafe { let _ = self.base.widget.as_mut().send(SCI_SETCODEPAGE, cp as u32, 0); }
    }
    fn codepage(&self) -> super::Codepage {
        unsafe { (self.base.widget.as_ref().send(SCI_GETCODEPAGE, 0, 0) as isize).into() }
    }
    fn append_text(&mut self, text: &str) {
        self.set_codepage(super::Codepage::Utf8);
        let len = text.len();
        let tptr = text.as_bytes().as_ptr();
        unsafe { self.base.widget.as_mut().send(SCI_APPENDTEXT, len as c_uint, tptr as c_int); }
    }
    fn on_ui_update(&mut self, cb: Option<scintilla_dev::Custom>) {
        self.ui_cb = cb;
    }
}

impl HasLayoutInner for ScintillaQt {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for ScintillaQt {
    fn parent(&self) -> Option<&controls::Member> {
        self.base.parent()
    }
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.parent_mut()
    }
    fn root(&self) -> Option<&controls::Member> {
        self.base.root()
    }
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.root_mut()
    }
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.measure(member, control, pw, ph);
        self.base.dirty = false;
        self.draw(member, control, Some((x, y)));
    }
    fn on_removed_from_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, _: &controls::Container) {}
}

impl MemberInner for ScintillaQt {
    type Id = QtId;

    fn on_set_visibility(&mut self, base: &mut MemberBase) {
        self.base.set_visibility(base.visibility);
        self.base.invalidate()
    }
    fn size(&self) -> (u16, u16) {
        self.base.measured_size
    }
    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.as_ref() as *const _ as *mut QWidget)
    }
}

impl Drawable for ScintillaQt {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase, coords: Option<(i32, i32)>) {
        self.base.draw(member, control, coords);
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = self.base.measured_size;
        self.base.measured_size = match member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => 42, // TODO min size
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => 42, // TODO min size
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            }
        };
        self.base.dirty = self.base.measured_size != old_size;
        (self.base.measured_size.0, self.base.measured_size.1, self.base.dirty)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate()
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
    use super::NewScintilla;

    Scintilla::new().into_control()
}

fn event_handler(object: &mut QObject, event: &QEvent) -> bool {
    unsafe {
        let ptr = object.property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
        if ptr != 0 {
            let sc: &mut Scintilla = mem::transmute(ptr);
            match event.type_() {
                QEventType::Resize => {
                    if sc.as_inner().as_inner().base.dirty {
                        sc.as_inner_mut().as_inner_mut().base.dirty = false;
                        let (width, height) = sc.as_inner().as_inner().size();
                        sc.call_on_resize(width, height);
                    }
                }
                _ => {}
            }
            if let Some(ref mut cb) = sc.as_inner_mut().as_inner_mut().ui_cb {
                let mut sc2: &mut Scintilla = mem::transmute(ptr);
                (cb.as_mut())(sc2);
            }
        }
        false
    }
}
impl_all_defaults!(Scintilla);
