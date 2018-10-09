use super::*;

use plygui_cocoa::common::*;

use std::os::raw::{c_int, c_void};

lazy_static! {
    static ref WINDOW_CLASS: RefClass = unsafe {
        register_window_class("PlyguiScintilla", BASE_CLASS, |decl| {
            decl.add_method(sel!(setFrameSize:), set_frame_size as extern "C" fn(&mut Object, Sel, NSSize));
        })
    };
}

const BASE_CLASS: &str = "ScintillaView";

#[repr(C)]
pub struct ConsoleCocoa {
    base: CocoaControlBase<Console>,

    fn_ptr: Option<extern "C" fn(*mut c_void, c_int, c_int, c_int) -> c_int>,
    self_ptr: Option<*mut c_void>,
}

impl ConsoleCocoa {
    pub fn new() -> Self {
        ConsoleCocoa {
            base: CocoaControlBase::with_params(*WINDOW_CLASS),
            fn_ptr: None,
            self_ptr: None,
        }
    }
    pub fn append_text(&mut self, text: &str) {
        self.set_codepage(super::Codepage::Utf8);
        if let Some(fn_ptr) = self.fn_ptr {
            let len = text.len();
            let tptr = text.as_bytes().as_ptr();
            (fn_ptr)(self.self_ptr.unwrap(), super::scintilla_sys::SCI_APPENDTEXT as i32, len as c_int, tptr as c_int);
        }
    }
    fn set_codepage(&mut self, cp: Codepage) {
        if let Some(fn_ptr) = self.fn_ptr {
            ((fn_ptr)(self.self_ptr.unwrap(), scintilla_sys::SCI_SETCODEPAGE as i32, cp as isize as i32, 0) as isize);
        }
    }
}

impl ControlInner for ConsoleCocoa {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &controls::Container, _x: i32, _y: i32, pw: u16, ph: u16) {
        unsafe {
            use scintilla_sys::{SCI_GETDIRECTFUNCTION, SCI_GETDIRECTPOINTER};

            let selfptr = member as *mut _ as *mut ::std::os::raw::c_void;
            (&mut *self.base.control).set_ivar(IVAR, selfptr);

            let fn_ptr: extern "C" fn(*mut c_void, c_int, c_int, c_int) -> c_int = msg_send![self.base.control, message:SCI_GETDIRECTFUNCTION wParam:0 lParam:0];
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

impl HasLayoutInner for ConsoleCocoa {
    fn on_layout_changed(&mut self, _: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl Drawable for ConsoleCocoa {
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

impl MemberInner for ConsoleCocoa {
    type Id = CocoaId;

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
    use plygui_api::controls::Member;

    unsafe {
        let sp = member_from_cocoa_id_mut::<Console>(this).unwrap();
        let () = msg_send![super(sp.native_id() as cocoa_id, Class::get(BASE_CLASS).unwrap()), setFrameSize: param];
        sp.call_on_resize(param.width as u16, param.height as u16);
    }
}
impl_all_defaults!(Console);
