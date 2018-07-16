use super::development as scintilla_dev;

use plygui_api::{layout, types, utils, controls};
use plygui_api::development::*;		
		
use plygui_win32::common;
use scintilla_sys::{Scintilla_RegisterClasses, Scintilla_ReleaseResources};

use winapi::shared::windef;
use winapi::shared::minwindef;
use winapi::um::winuser;
use winapi::um::commctrl;
use winapi::ctypes::c_void as win_void;

use std::{ptr, mem};
use std::os::windows::ffi::OsStrExt;
use std::os::raw::{c_void, c_int};
use std::ffi::OsStr;
use std::cmp::max;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static GLOBAL_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

lazy_static! {
	pub static ref WINDOW_CLASS: Vec<u16> = OsStr::new("Scintilla")
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();
}

pub type Scintilla = Member<Control<ScintillaWin32>>;

#[repr(C)]
pub struct ScintillaWin32 {
    base: common::WindowsControlBase<Scintilla>,
    
    ui_cb: Option<scintilla_dev::Custom>,
    fn_ptr: Option<extern "C" fn(*mut c_void, c_int, c_int, c_int) -> c_int>,
    self_ptr: Option<*mut c_void>,
}

impl scintilla_dev::ScintillaInner for ScintillaWin32 {
	fn new() -> Box<Scintilla> {
		if GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst) < 1 {
            unsafe { 
                if Scintilla_RegisterClasses(common::hinstance() as *mut c_void) == 0 {
                    panic!("Cannot register Scintilla Win32 class");
                }
            }
        }
        let b: Box<Scintilla> = Box::new(Member::with_inner(Control::with_inner(
        		ScintillaWin32 {
		            base: common::WindowsControlBase::new(),
		            ui_cb: None,
		            fn_ptr: None,
		            self_ptr: None,
		        }, ()),
        		MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        //b.set_layout_padding(layout::BoundarySize::AllTheSame(DEFAULT_PADDING).into());
        b
	}
	fn on_ui_update(&mut self, cb: Option<scintilla_dev::Custom>) {
	    self.ui_cb = cb;
	}
	fn with_content(content: &str) -> Box<Scintilla> {
	    use Scintilla;
	    
		let mut b = Self::new();
		b.append_text(content);
		b
	}
	fn set_margin_width(&mut self, index: usize, width: isize) {
		if let Some(fn_ptr) = self.fn_ptr {
            (fn_ptr)(self.self_ptr.unwrap(), super::scintilla_sys::SCI_SETMARGINWIDTHN as i32, index as c_int, width as c_int);
        }
	}
	fn set_readonly(&mut self, readonly: bool) {
	    if let Some(fn_ptr) = self.fn_ptr {
            (fn_ptr)(self.self_ptr.unwrap(), super::scintilla_sys::SCI_SETREADONLY as i32, if readonly {1} else {0}, 0);
        }
	}
    fn is_readonly(&self) -> bool {
        if let Some(fn_ptr) = self.fn_ptr {
            if (fn_ptr)(self.self_ptr.unwrap(), super::scintilla_sys::SCI_GETREADONLY as i32, 0, 0) == 0 { false } else { true }
        } else {
            true
        }
    }
    fn set_codepage(&mut self, cp: super::Codepage) {
        if let Some(fn_ptr) = self.fn_ptr {
            ((fn_ptr)(self.self_ptr.unwrap(), super::scintilla_sys::SCI_SETCODEPAGE as i32, cp as isize as i32, 0) as isize);
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
            (fn_ptr)(self.self_ptr.unwrap(), super::scintilla_sys::SCI_APPENDTEXT as i32, len as c_int, tptr as c_int);
        }
    }
}

impl ControlInner for ScintillaWin32 {
	fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container, x: i32, y: i32) {
		let selfptr = base as *mut _ as *mut win_void;
        let (pw, ph) = parent.size();
        //let (lp,tp,rp,bp) = base.control.layout.padding.into();
        let (lm, tm, rm, bm) = base.control.layout.margin.into();
        let (hwnd, id) = unsafe {
            self.base.hwnd = parent.native_id() as windef::HWND; // required for measure, as we don't have own hwnd yet
            let (w, h, _) = self.measure(base, pw, ph);
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
        
        unsafe {
            self.fn_ptr = Some(mem::transmute(winuser::SendMessageW(self.base.hwnd, super::scintilla_sys::SCI_GETDIRECTFUNCTION, 0, 0)));
            self.self_ptr = Some(winuser::SendMessageW(self.base.hwnd, super::scintilla_sys::SCI_GETDIRECTPOINTER, 0, 0) as *mut c_void);
        }
	}
    fn on_removed_from_container(&mut self, _: &mut MemberControlBase, _: &controls::Container) {
    	common::destroy_hwnd(self.base.hwnd, self.base.subclass_id, Some(handler));
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
        if GLOBAL_COUNT.fetch_sub(1, Ordering::SeqCst) < 1 {
            unsafe { Scintilla_ReleaseResources(); }
        }
    }
}

impl HasLayoutInner for ScintillaWin32 {
	fn on_layout_changed(&mut self, base: &mut MemberBase) {
		let hwnd = self.base.hwnd;
        if !hwnd.is_null() {
        	let base = self.cast_base_mut(base);
        	self.invalidate(base);
		}
	}
}

impl MemberInner for ScintillaWin32 {
	type Id = common::Hwnd;
	
	fn size(&self) -> (u16, u16) {
        let rect = unsafe { common::window_rect(self.base.hwnd) };
        (
            (rect.right - rect.left) as u16,
            (rect.bottom - rect.top) as u16,
        )
    }

    fn on_set_visibility(&mut self, base: &mut MemberBase) {
	    let hwnd = self.base.hwnd;
        if !hwnd.is_null() {
        	unsafe {
	            winuser::ShowWindow(
	                self.base.hwnd,
	                if base.visibility == types::Visibility::Visible {
	                    winuser::SW_SHOW
	                } else {
	                    winuser::SW_HIDE
	                },
	            );
	        }
			self.invalidate(utils::member_control_base_mut(common::member_from_hwnd::<Scintilla>(hwnd)));
	    }
    }
    unsafe fn native_id(&self) -> Self::Id {
        self.base.hwnd.into()
    }
}

impl Drawable for ScintillaWin32 {
	fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>) {
		if coords.is_some() {
            self.base.coords = coords;
        }
        //let (lp,tp,rp,bp) = base.control.layout.padding.into();
        let (lm, tm, rm, bm) = base.control.layout.margin.into();
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
    fn measure(&mut self, base: &mut MemberControlBase, w: u16, h: u16) -> (u16, u16, bool) {
    	let old_size = self.base.measured_size;
        let (lp,tp,rp,bp) = base.control.layout.padding.into();
        let (lm, tm, rm, bm) = base.control.layout.margin.into();
        
        self.base.measured_size = match base.member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match base.control.layout.width {
                    layout::Size::MatchParent => w,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => {
                        42 as u16 // TODO min_width
                    } 
                };
                let h = match base.control.layout.height {
                    layout::Size::MatchParent => h,
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
    fn invalidate(&mut self, base: &mut MemberControlBase) {
    	self.base.invalidate(base)
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

            if let Some(ref mut cb) = sc.base_mut().handler_resize {
                let mut sc2: &mut Scintilla = mem::transmute(param);
                (cb.as_mut())(sc2, width, height);
            }
        }
        _ => {}
    }
    if let Some(ref mut cb) = sc.as_inner_mut().as_inner_mut().ui_cb {
        let mut sc2: &mut Scintilla = mem::transmute(param);
        (cb.as_mut())(sc2);
    }
    commctrl::DefSubclassProc(hwnd, msg, wparam, lparam)
}

impl_all_defaults!(Scintilla);
