use super::development as scintilla_dev;
use super::*;

use plygui_cocoa::common::*;

use std::os::raw::{c_int, c_void, c_ulong, c_long};

lazy_static! {
    static ref WINDOW_CLASS: common::RefClass = unsafe {
        common::register_window_class("PlyguiConsole", BASE_CLASS, |decl| {
            decl.add_method(sel!(setFrameSize:), set_frame_size as extern "C" fn(&mut Object, Sel, NSSize));
        })
    };
}

pub type Scintilla = Member<Control<ScintillaCocoa>>;

const BASE_CLASS: &str = "ScintillaView";

#[repr(C)]
pub struct ScintillaCocoa {
    base: common::CocoaControlBase<Scintilla>,

    fn_ptr: Option<extern "C" fn(*mut c_void, c_int, c_ulong, c_long) -> *mut c_void>,
    self_ptr: Option<*mut c_void>,
}

impl scintilla_dev::ScintillaInner for ScintillaCocoa {
    fn new() -> Box<super::Scintilla> {
        let mut b = Box::new(Member::with_inner(
            Control::with_inner(
                ScintillaCocoa {
                    base: common::CocoaControlBase::with_params(*WINDOW_CLASS),
                    fn_ptr: None,
                    self_ptr: None,
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));

        unsafe {
            let selfptr = b.as_mut() as *mut _ as *mut ::std::os::raw::c_void;
            (&mut *b.as_inner_mut().as_inner_mut().base.control).set_ivar(common::IVAR, selfptr);
        }
        b
    }

    fn set_margin_width(&mut self, index: usize, width: isize) {
        if let Some(fn_ptr) = self.fn_ptr {
            (fn_ptr)(self.self_ptr.unwrap(), scintilla_sys::SCI_SETMARGINWIDTHN as i32, index as c_ulong, width as c_long);
        }
    }
    fn set_readonly(&mut self, readonly: bool) {
        if let Some(fn_ptr) = self.fn_ptr {
            (fn_ptr)(self.self_ptr.unwrap(), scintilla_sys::SCI_SETREADONLY as i32, if readonly { 1 } else { 0 }, 0);
        }
    }
    fn is_readonly(&self) -> bool {
        if let Some(fn_ptr) = self.fn_ptr {
            !(fn_ptr)(self.self_ptr.unwrap(), scintilla_sys::SCI_GETREADONLY as i32, 0, 0).is_null()
        } else {
            true
        }
    }
    fn set_codepage(&mut self, cp: Codepage) {
        if let Some(fn_ptr) = self.fn_ptr {
            ((fn_ptr)(self.self_ptr.unwrap(), scintilla_sys::SCI_SETCODEPAGE as i32, cp as c_ulong, 0) as isize);
        }
    }
    fn codepage(&self) -> super::Codepage {
        if let Some(fn_ptr) = self.fn_ptr {
            ((fn_ptr)(self.self_ptr.unwrap(), scintilla_sys::SCI_GETCODEPAGE as i32, 0, 0) as isize).into()
        } else {
            Default::default()
        }
    }
    fn append_text(&mut self, text: &str) {
        self.set_codepage(super::Codepage::Utf8);
        if let Some(fn_ptr) = self.fn_ptr {
            let len = text.len();
            let tptr = text.as_bytes().as_ptr();
            (fn_ptr)(self.self_ptr.unwrap(), super::scintilla_sys::SCI_APPENDTEXT as i32, len as c_ulong, tptr as c_long);
        }
    }
}

impl ControlInner for ScintillaCocoa {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &controls::Container, _x: i32, _y: i32, pw: u16, ph: u16) {
        unsafe {
            use scintilla_sys::{SCI_GETDIRECTFUNCTION, SCI_GETDIRECTPOINTER};

            let fn_ptr: extern "C" fn(*mut c_void, c_int, c_ulong, c_long) -> *mut c_void = msg_send![self.base.control, message:SCI_GETDIRECTFUNCTION wParam:0 lParam:0];
            let self_ptr: *mut c_void = msg_send![self.base.control, message:SCI_GETDIRECTPOINTER wParam:0 lParam:0];

            self.fn_ptr = Some(fn_ptr);
            self.self_ptr = Some(self_ptr);
        }
        self.measure(member, control, pw, ph);
    }
    fn on_removed_from_container(&mut self, _: &mut MemberBase, _: &mut ControlBase, _: &controls::Container) {
        self.fn_ptr = None;
        self.self_ptr = None;
        unsafe {
            self.base.on_removed_from_container();
        }
    }

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

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        fill_from_markup_base!(self, base, markup, registry, Scintilla, ["Scintilla"]);
    }
}

impl HasLayoutInner for ScintillaCocoa {
    fn on_layout_changed(&mut self, _: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl Drawable for ScintillaCocoa {
    fn draw(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, coords: Option<(i32, i32)>) {
        self.base.draw(coords);
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = self.base.measured_size;
        self.base.measured_size = match member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => {
                        42 as u16 // TODO min_width
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => {
                        42 as u16 // TODO min_height
                    }
                };
                (w, h)
            }
        };
        (self.base.measured_size.0, self.base.measured_size.1, self.base.measured_size != old_size)
    }
    fn invalidate(&mut self, _: &mut MemberBase, _: &mut ControlBase) {
        self.base.invalidate();
    }
}

impl MemberInner for ScintillaCocoa {
    type Id = common::CocoaId;

    fn size(&self) -> (u16, u16) {
        self.base.measured_size
    }

    fn on_set_visibility(&mut self, base: &mut MemberBase) {
        self.base.on_set_visibility(base);
    }

    unsafe fn native_id(&self) -> Self::Id {
        self.base.control.into()
    }
}
extern "C" fn set_frame_size(this: &mut Object, _: Sel, param: NSSize) {
    unsafe {
        let sp = common::member_from_cocoa_id_mut::<Scintilla>(this).unwrap();
        let () = msg_send![super(sp.as_inner_mut().as_inner_mut().base.control, Class::get(BASE_CLASS).unwrap()), setFrameSize: param];
        sp.call_on_resize(param.width as u16, param.height as u16);
    }
}
impl_all_defaults!(Scintilla);
