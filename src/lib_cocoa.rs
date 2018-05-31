use super::*;
use super::development as scintilla_dev;

use plygui_api::{layout, types, development, controls};
use plygui_api::development::HasInner;
use plygui_cocoa::common;

use self::cocoa::foundation::{NSRect, NSSize, NSPoint};
use self::cocoa::base::id;

use std::mem;
use std::cmp::max;
use std::os::raw::{c_void, c_int};

lazy_static! {
	static ref WINDOW_CLASS: common::RefClass = unsafe { common::register_window_class("PlyguiScintilla","ScintillaView",|_|{}) };
}

pub type Scintilla = development::Member<development::Control<ScintillaCocoa>>;

#[repr(C)]
pub struct ScintillaCocoa {
    base: common::CocoaControlBase<Scintilla>,
    
    fn_ptr: Option<extern "C" fn(*mut c_void, c_int, c_int, c_int)>,
    self_ptr: Option<*mut c_void>,
}

impl ScintillaCocoa {
	
}

impl scintilla_dev::ScintillaInner for ScintillaCocoa {
	fn new() -> Box<super::Scintilla> {
		let mut b = Box::new(development::Member::with_inner(development::Control::with_inner(ScintillaCocoa {
                     base: common::CocoaControlBase::with_params(*WINDOW_CLASS),
                     fn_ptr: None,
				     self_ptr: None,
                 }, ()), development::MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut)));
		
        unsafe {
        	use scintilla_sys::{SCI_GETDIRECTFUNCTION, SCI_GETDIRECTPOINTER};
        	
	        let selfptr = b.as_mut() as *mut _ as *mut ::std::os::raw::c_void;
        	
	        (&mut *b.as_inner_mut().as_inner_mut().base.control).set_ivar(common::IVAR, selfptr);
        	
        	let fn_ptr: id = msg_send![b.as_inner_mut().as_inner_mut().base.control, message:SCI_GETDIRECTFUNCTION wParam:0 lParam:0];
        	let self_ptr: id = msg_send![b.as_inner_mut().as_inner_mut().base.control, message:SCI_GETDIRECTPOINTER wParam:0 lParam:0];
        	
            b.as_inner_mut().as_inner_mut().fn_ptr = Some(mem::transmute(fn_ptr));
            b.as_inner_mut().as_inner_mut().self_ptr = Some(mem::transmute(self_ptr));
        }
        b
	}
	fn with_content(content: &str) -> Box<super::Scintilla> {
		let mut b = Box::new(development::Member::with_inner(development::Control::with_inner(ScintillaCocoa {
                     base: common::CocoaControlBase::with_params(*WINDOW_CLASS),
                     fn_ptr: None,
				     self_ptr: None,
                 }, ()), development::MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut)));
		
        unsafe {
        	use scintilla_sys::{SCI_GETDIRECTFUNCTION, SCI_GETDIRECTPOINTER};
        	
	        let selfptr = b.as_mut() as *mut _ as *mut ::std::os::raw::c_void;
        	
	        (&mut *b.as_inner_mut().as_inner_mut().base.control).set_ivar(common::IVAR, selfptr);
        	
        	let fn_ptr: id = msg_send![b.as_inner_mut().as_inner_mut().base.control, message:SCI_GETDIRECTFUNCTION wParam:0 lParam:0];
        	let self_ptr: id = msg_send![b.as_inner_mut().as_inner_mut().base.control, message:SCI_GETDIRECTPOINTER wParam:0 lParam:0];
        	
            b.as_inner_mut().as_inner_mut().fn_ptr = Some(mem::transmute(fn_ptr));
            b.as_inner_mut().as_inner_mut().self_ptr = Some(mem::transmute(self_ptr));
        }
        // TODO content!
        b
	}
	fn set_margin_width(&mut self, index: usize, width: isize) {
		if let Some(fn_ptr) = self.fn_ptr {
            (fn_ptr)(self.self_ptr.unwrap(), scintilla_sys::SCI_SETMARGINWIDTHN as i32, index as c_int, width as c_int);
        }
	}
}

impl development::ControlInner for ScintillaCocoa {
	fn on_added_to_container(&mut self, base: &mut development::MemberControlBase, parent: &controls::Container, x: i32, y: i32) {
		use plygui_api::development::Drawable;
    	
        let (pw, ph) = parent.draw_area_size();
        let _ = self.measure(base, pw, ph);
	}
    fn on_removed_from_container(&mut self, _: &mut development::MemberControlBase, _: &controls::Container) {
    	unsafe { self.base.on_removed_from_container(); }
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
    fn fill_from_markup(&mut self, base: &mut development::MemberControlBase, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
    	fill_from_markup_base!(self, base, markup, registry, Scintilla, ["Scintilla"]);
    }
}

impl development::HasLayoutInner for ScintillaCocoa {
	fn on_layout_changed(&mut self, _: &mut development::MemberBase) {
		self.base.invalidate();
	}
}


impl development::Drawable for ScintillaCocoa {
	fn draw(&mut self, base: &mut development::MemberControlBase, coords: Option<(i32, i32)>) {
    	if coords.is_some() {
    		self.base.coords = coords;
    	}
    	if let Some((x, y)) = self.base.coords {
    		use plygui_api::development::ControlInner;
    		
    		let (_,ph) = self.parent().unwrap().size();
    		unsafe {
	            let mut frame: NSRect = self.base.frame();
	            frame.size = NSSize::new(self.base.measured_size.0 as f64,
	                                     self.base.measured_size.1 as f64);
	            frame.origin = NSPoint::new(x as f64, (ph as i32 - y - self.base.measured_size.1 as i32) as f64);
	            msg_send![self.base.control, setFrame: frame];
	        }
    		if let Some(ref mut cb) = base.member.handler_resize {
	            unsafe {
	                let mut ll2 = common::member_from_cocoa_id_mut::<Scintilla>(self.base.control).unwrap();
	                (cb.as_mut())(ll2, self.base.measured_size.0, self.base.measured_size.1);
	            }
	        }
    	}
    }
    fn measure(&mut self, base: &mut development::MemberControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
    	let old_size = self.base.measured_size;
        let (lp, tp, rp, bp) = base.control.layout.padding.into();
        let (lm, tm, rm, bm) = base.control.layout.margin.into();

        self.base.measured_size = match base.member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match base.control.layout.width {
                    layout::Size::MatchParent => parent_width,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => {
                        42 as u16 // TODO min_width
                    } 
                };
                let h = match base.control.layout.height {
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
    fn invalidate(&mut self, _: &mut development::MemberControlBase) {
    	self.base.invalidate();
    }
}

impl development::MemberInner for ScintillaCocoa {
	type Id = common::CocoaId;
	
    fn size(&self) -> (u16, u16) {
    	self.base.measured_size
    }
    
    fn on_set_visibility(&mut self, base: &mut development::MemberBase) {
    	self.base.on_set_visibility(base);
    }
    
    unsafe fn native_id(&self) -> Self::Id {
    	self.base.control.into()
    }
}

impl_all_defaults!(Scintilla);
