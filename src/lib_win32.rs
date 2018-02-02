use plygui_api::{layout, types, development, callbacks};
use plygui_api::traits::{UiControl, UiClickable, UiHasLabel, UiHasLayout, UiButton, UiMember, UiContainer};
use plygui_api::members::MEMBER_ID_BUTTON;

use plygui_win32::common;
use scintilla_sys::{Scintilla_RegisterClasses, Scintilla_ReleaseResources};

use winapi::shared::windef;
use winapi::shared::minwindef;
use winapi::um::winuser;
use winapi::um::wingdi;
use winapi::um::commctrl;
use winapi::ctypes::c_void;

use std::{ptr, mem, str};
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use std::borrow::Cow;
use std::cmp::max;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static GLOBAL_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

pub const CLASS_ID: &str = "Scintilla";

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
            //unsafe { Scintilla_RegisterClasses() }
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
        common::destroy_hwnd(self.base.hwnd, 0, None);
        if GLOBAL_COUNT.fetch_sub(1, Ordering::SeqCst) < 1 {
            unsafe { Scintilla_ReleaseResources(); }
        }
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
                //(cb.as_mut())(sc2, width, height);
            }
        }
        _ => {}
    }

    commctrl::DefSubclassProc(hwnd, msg, wparam, lparam)
}

impl_invalidate!(Scintilla);
impl_is_control!(Scintilla);
impl_size!(Scintilla);
impl_member_id!(super::MEMBER_ID_SCINTILLA);