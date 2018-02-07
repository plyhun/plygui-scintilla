use super::*;

use plygui_api::{layout, types, development, callbacks};
use plygui_api::traits::{UiControl, UiHasLayout, UiMember, UiContainer};

use plygui_win32::common;
use scintilla_sys::{Scintilla_RegisterClasses, Scintilla_ReleaseResources};

use winapi::shared::windef;
use winapi::shared::minwindef;
use winapi::um::winuser;
use winapi::um::commctrl;
use winapi::ctypes::c_void;

use std::{ptr, mem, str};
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use std::cmp::max;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static GLOBAL_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

pub const CLASS_ID: &str = MEMBER_ID_SCINTILLA;

lazy_static! {
	pub static ref WINDOW_CLASS: Vec<u16> = OsStr::new(CLASS_ID)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();
}

#[repr(C)]
pub struct Scintilla {
    base: common::WindowsControlBase,
    
}

impl Scintilla {
    pub fn new() -> Box<Scintilla> {
        if GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst) < 1 {
            unsafe { 
                if Scintilla_RegisterClasses(common::hinstance() as *mut std::os::raw::c_void) == 0 {
                    panic!("Cannot register Scintilla Win32 class");
                }
            }
        }
        let b = Box::new(Scintilla {
            base: common::WindowsControlBase::with_params(
                invalidate_impl,
                development::UiMemberFunctions {
                    fn_member_id: member_id,
                    fn_is_control: is_control,
                    fn_is_control_mut: is_control_mut,
                    fn_size: size,
                },
            ),
            
        });

        b
    }
}

impl Drop for Scintilla {
    fn drop(&mut self) {
        //self.set_visibility(types::Visibility::Gone);
        if GLOBAL_COUNT.fetch_sub(1, Ordering::SeqCst) < 1 {
            unsafe { Scintilla_ReleaseResources(); }
        }
    }
}

impl UiScintilla for Scintilla {
    
}
impl UiControl for Scintilla {
    fn on_added_to_container(&mut self, parent: &UiContainer, x: i32, y: i32) {
        use plygui_api::development::UiDrawable;

        let selfptr = self as *mut _ as *mut c_void;
        let (pw, ph) = parent.size();
        //let (lp,tp,rp,bp) = self.base.control_base.layout.padding.into();
        let (lm, tm, rm, bm) = self.base.control_base.layout.margin.into();
        let (hwnd, id) = unsafe {
            self.base.hwnd = parent.native_id() as windef::HWND; // required for measure, as we don't have own hwnd yet
            let (w, h, _) = self.measure(pw, ph);
            common::create_control_hwnd(
                x as i32 + lm,
                y as i32 + tm,
                w as i32 - rm,
                h as i32 - bm,
                parent.native_id() as windef::HWND,
                0,
                WINDOW_CLASS.as_ptr(),
                "",
                winuser::BS_PUSHBUTTON | winuser::WS_TABSTOP,
                selfptr,
                Some(handler),
            )
        };
        self.base.hwnd = hwnd;
        self.base.subclass_id = id;
    }
    fn on_removed_from_container(&mut self, _: &UiContainer) {
        common::destroy_hwnd(self.base.hwnd, self.base.subclass_id, Some(handler));
        self.base.hwnd = 0 as windef::HWND;
        self.base.subclass_id = 0;
    }

    fn is_container_mut(&mut self) -> Option<&mut UiContainer> {
        None
    }
    fn is_container(&self) -> Option<&UiContainer> {
        None
    }

    fn parent(&self) -> Option<&types::UiMemberBase> {
        self.base.parent()
    }
    fn parent_mut(&mut self) -> Option<&mut types::UiMemberBase> {
        self.base.parent_mut()
    }
    fn root(&self) -> Option<&types::UiMemberBase> {
        self.base.root()
    }
    fn root_mut(&mut self) -> Option<&mut types::UiMemberBase> {
        self.base.root_mut()
    }
    fn as_has_layout(&self) -> &UiHasLayout {
        self
    }
    fn as_has_layout_mut(&mut self) -> &mut UiHasLayout {
        self
    }

    /*#[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        
    }*/
}
impl UiHasLayout for Scintilla {
    fn layout_width(&self) -> layout::Size {
        self.base.control_base.layout.width
    }
    fn layout_height(&self) -> layout::Size {
        self.base.control_base.layout.height
    }
    fn layout_gravity(&self) -> layout::Gravity {
        self.base.control_base.layout.gravity
    }
    fn layout_alignment(&self) -> layout::Alignment {
        self.base.control_base.layout.alignment
    }
    fn layout_padding(&self) -> layout::BoundarySize {
        self.base.control_base.layout.padding
    }
    fn layout_margin(&self) -> layout::BoundarySize {
        self.base.control_base.layout.margin
    }

    fn set_layout_width(&mut self, width: layout::Size) {
        self.base.control_base.layout.width = width;
        self.base.invalidate();
    }
    fn set_layout_height(&mut self, height: layout::Size) {
        self.base.control_base.layout.height = height;
        self.base.invalidate();
    }
    fn set_layout_gravity(&mut self, gravity: layout::Gravity) {
        self.base.control_base.layout.gravity = gravity;
        self.base.invalidate();
    }
    fn set_layout_alignment(&mut self, alignment: layout::Alignment) {
        self.base.control_base.layout.alignment = alignment;
        self.base.invalidate();
    }
    fn set_layout_padding(&mut self, padding: layout::BoundarySizeArgs) {
        self.base.control_base.layout.padding = padding.into();
        self.base.invalidate();
    }
    fn set_layout_margin(&mut self, margin: layout::BoundarySizeArgs) {
        self.base.control_base.layout.margin = margin.into();
        self.base.invalidate();
    }
    fn as_member(&self) -> &UiMember {
        self
    }
    fn as_member_mut(&mut self) -> &mut UiMember {
        self
    }
}
impl UiMember for Scintilla {
    fn size(&self) -> (u16, u16) {
        let rect = unsafe { common::window_rect(self.base.hwnd) };
        (
            (rect.right - rect.left) as u16,
            (rect.bottom - rect.top) as u16,
        )
    }

    fn on_resize(&mut self, handler: Option<callbacks::Resize>) {
        self.base.h_resize = handler;
    }

    fn set_visibility(&mut self, visibility: types::Visibility) {
        self.base.control_base.member_base.visibility = visibility;
        unsafe {
            winuser::ShowWindow(
                self.base.hwnd,
                if self.base.control_base.member_base.visibility == types::Visibility::Invisible {
                    winuser::SW_HIDE
                } else {
                    winuser::SW_SHOW
                },
            );
            self.base.invalidate();
        }
    }
    fn visibility(&self) -> types::Visibility {
        self.base.control_base.member_base.visibility
    }

    fn is_control(&self) -> Option<&UiControl> {
        Some(self)
    }
    fn is_control_mut(&mut self) -> Option<&mut UiControl> {
        Some(self)
    }
    fn as_base(&self) -> &types::UiMemberBase {
        self.base.control_base.member_base.as_ref()
    }
    fn as_base_mut(&mut self) -> &mut types::UiMemberBase {
        self.base.control_base.member_base.as_mut()
    }

    unsafe fn native_id(&self) -> usize {
        self.base.hwnd as usize
    }
}
impl development::UiDrawable for Scintilla {
    fn draw(&mut self, coords: Option<(i32, i32)>) {
        if coords.is_some() {
            self.base.coords = coords;
        }
        //let (lp,tp,rp,bp) = self.base.control_base.layout.padding.into();
        let (lm, tm, rm, bm) = self.base.control_base.layout.margin.into();
        if let Some((x, y)) = self.base.coords {
            unsafe {
                winuser::SetWindowPos(
                    self.base.hwnd,
                    ptr::null_mut(),
                    x + lm,
                    y + tm,
                    self.base.measured_size.0 as i32 - rm,
                    self.base.measured_size.1 as i32 - bm,
                    0,
                );
            }
        }
    }
    fn measure(&mut self, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = self.base.measured_size;
        let (lp, tp, rp, bp) = self.base.control_base.layout.padding.into();
        let (lm, tm, rm, bm) = self.base.control_base.layout.margin.into();

        self.base.measured_size = match self.visibility() {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match self.layout_width() {
                    layout::Size::MatchParent => parent_width,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => {
                        42 as u16 // TODO min_width
                    } 
                };
                let h = match self.layout_height() {
                    layout::Size::MatchParent => parent_height,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => {
                        42 as u16 // TODO min_height
                    } 
                };
                (
                    max(0, w as i32 + lm + rm + lp + rp) as u16,
                    max(0, h as i32 + tm + bm + tp + bp) as u16,
                )
            },
        };
        (
            self.base.measured_size.0,
            self.base.measured_size.1,
            self.base.measured_size != old_size,
        )
    }
}

/*#[allow(dead_code)]
pub(crate) fn spawn() -> Box<UiControl> {
    Scintilla::new("")
}*/

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

            if let Some(ref mut cb) = sc.base.h_resize {
                let mut sc2: &mut Scintilla = mem::transmute(param);
                (cb.as_mut())(sc2, width, height);
            }
        }
        _ => {}
    }

    commctrl::DefSubclassProc(hwnd, msg, wparam, lparam)
}

impl_invalidate!(Scintilla);
impl_is_control!(Scintilla);
impl_size!(Scintilla);
impl_member_id!(MEMBER_ID_SCINTILLA);