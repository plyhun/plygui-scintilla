use super::development as scintilla_dev;

use plygui_win32::common::*;
use scintilla_sys::{Scintilla_RegisterClasses, Scintilla_ReleaseResources};

use std::os::raw::{c_int, c_void as r_void, c_ulong, c_long};
use std::sync::atomic::Ordering;

lazy_static! {
    pub static ref WINDOW_CLASS: Vec<u16> = OsStr::new("Scintilla").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
}

pub type Scintilla = Member<Control<ScintillaWin32>>;

#[repr(C)]
pub struct ScintillaWin32 {
    base: WindowsControlBase<Scintilla>,

    fn_ptr: Option<extern "C" fn(*mut r_void, c_int, c_ulong, c_long) -> *mut r_void>,
    self_ptr: Option<*mut r_void>,
}

impl scintilla_dev::ScintillaInner for ScintillaWin32 {
    fn new() -> Box<Scintilla> {
        if scintilla_dev::GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst) < 1 {
            unsafe {
                if Scintilla_RegisterClasses(hinstance() as *mut r_void) == 0 {
                    panic!("Cannot register Scintilla Win32 class");
                }
            }
        }
        let b: Box<Scintilla> = Box::new(Member::with_inner(
            Control::with_inner(
                ScintillaWin32 {
                    base: WindowsControlBase::new(),
                    fn_ptr: None,
                    self_ptr: None,
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        b
    }
    fn set_margin_width(&mut self, index: usize, width: isize) {
        if let Some(fn_ptr) = self.fn_ptr {
            (fn_ptr)(self.self_ptr.unwrap(), super::scintilla_sys::SCI_SETMARGINWIDTHN as i32, index as c_ulong, width as c_long);
        }
    }
    fn set_readonly(&mut self, readonly: bool) {
        if let Some(fn_ptr) = self.fn_ptr {
            (fn_ptr)(self.self_ptr.unwrap(), super::scintilla_sys::SCI_SETREADONLY as i32, if readonly { 1 } else { 0 }, 0);
        }
    }
    fn is_readonly(&self) -> bool {
        if let Some(fn_ptr) = self.fn_ptr {
            !(fn_ptr)(self.self_ptr.unwrap(), super::scintilla_sys::SCI_GETREADONLY as i32, 0, 0).is_null()
        } else {
            true
        }
    }
    fn set_codepage(&mut self, cp: super::Codepage) {
        if let Some(fn_ptr) = self.fn_ptr {
            ((fn_ptr)(self.self_ptr.unwrap(), super::scintilla_sys::SCI_SETCODEPAGE as i32, cp as c_ulong, 0) as isize);
        }
    }
    fn codepage(&self) -> super::Codepage {
        if let Some(fn_ptr) = self.fn_ptr {
            ((fn_ptr)(self.self_ptr.unwrap(), super::scintilla_sys::SCI_GETCODEPAGE as i32, 0, 0) as isize).into()
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

impl ControlInner for ScintillaWin32 {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        let selfptr = member as *mut _ as *mut c_void;
        let (hwnd, id) = unsafe {
            self.base.hwnd = parent.native_id() as windef::HWND; // required for measure, as we don't have own hwnd yet
            let (w, h, _) = self.measure(member, control, pw, ph);
            create_control_hwnd(x as i32, y as i32, w as i32, h as i32, parent.native_id() as windef::HWND, 0, WINDOW_CLASS.as_ptr(), "", winuser::BS_PUSHBUTTON | winuser::WS_TABSTOP, selfptr, Some(handler))
        };
        self.base.hwnd = hwnd;
        self.base.subclass_id = id;

        unsafe {
            self.fn_ptr = Some(mem::transmute(winuser::SendMessageW(self.base.hwnd, super::scintilla_sys::SCI_GETDIRECTFUNCTION, 0, 0)));
            self.self_ptr = Some(winuser::SendMessageW(self.base.hwnd, super::scintilla_sys::SCI_GETDIRECTPOINTER, 0, 0) as *mut r_void);
        }
    }
    fn on_removed_from_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, _: &controls::Container) {
        destroy_hwnd(self.base.hwnd, self.base.subclass_id, Some(handler));
        self.base.hwnd = 0 as windef::HWND;
        self.base.subclass_id = 0;
        self.fn_ptr = None;
        self.self_ptr = None;
    }

    fn parent(&self) -> Option<&controls::Member> {
        self.base.parent().map(|p| p.as_member())
    }
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.parent_mut().map(|p| p.as_member_mut())
    }
    fn root(&self) -> Option<&controls::Member> {
        self.base.root().map(|p| p.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.root_mut().map(|p| p.as_member_mut())
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, base: &mut development::MemberControlBase, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        fill_from_markup_base!(self, base, markup, registry, Scintilla, ["Scintilla"]);
    }
}

impl Drop for ScintillaWin32 {
    fn drop(&mut self) {
        if scintilla_dev::GLOBAL_COUNT.fetch_sub(1, Ordering::SeqCst) < 1 {
            unsafe {
                Scintilla_ReleaseResources();
            }
        }
    }
}

impl HasLayoutInner for ScintillaWin32 {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        let hwnd = self.base.hwnd;
        if !hwnd.is_null() {
            self.base.invalidate();
        }
    }
}

impl MemberInner for ScintillaWin32 {
    type Id = Hwnd;
    
    fn size(&self) -> (u16, u16) {
        let rect = unsafe { window_rect(self.base.hwnd) };
        ((rect.right - rect.left) as u16, (rect.bottom - rect.top) as u16)
    }

    fn on_set_visibility(&mut self, base: &mut MemberBase) {
        let hwnd = self.base.hwnd;
        if !hwnd.is_null() {
            unsafe {
                winuser::ShowWindow(self.base.hwnd, if base.visibility == types::Visibility::Visible { winuser::SW_SHOW } else { winuser::SW_HIDE });
            }
            self.base.invalidate();
        }
    }
    unsafe fn native_id(&self) -> Self::Id {
        self.base.hwnd.into()
    }
}

impl Drawable for ScintillaWin32 {
    fn draw(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, coords: Option<(i32, i32)>) {
        if coords.is_some() {
            self.base.coords = coords;
        }
        if let Some((x, y)) = self.base.coords {
            unsafe {
                winuser::SetWindowPos(self.base.hwnd, ptr::null_mut(), x, y, self.base.measured_size.0 as i32, self.base.measured_size.1 as i32, 0);
            }
        }
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        let old_size = self.base.measured_size;
        self.base.measured_size = match member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => w,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => {
                        defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING // TODO min_width
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => h,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => {
                        defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING // TODO min_height
                    }
                };
                (cmp::max(0, w as i32) as u16, cmp::max(0, h as i32) as u16)
            }
        };
        (self.base.measured_size.0, self.base.measured_size.1, self.base.measured_size != old_size)
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

unsafe extern "system" fn handler(hwnd: windef::HWND, msg: minwindef::UINT, wparam: minwindef::WPARAM, lparam: minwindef::LPARAM, _: usize, param: usize) -> isize {
    let sc: &mut Scintilla = mem::transmute(param);
    let ww = winuser::GetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA);
    if ww == 0 {
        winuser::SetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA, param as isize);
    }
    match msg {
        winuser::WM_SIZE => {
            let width = lparam as u16;
            let height = (lparam >> 16) as u16;

            sc.call_on_resize(width, height);
        }
        _ => {}
    }
    commctrl::DefSubclassProc(hwnd, msg, wparam, lparam)
}

impl_all_defaults!(Scintilla);
