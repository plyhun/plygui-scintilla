use super::*;

use plygui_api::{layout, types, development, callbacks};
use plygui_api::traits::{UiControl, UiHasLayout, UiMember, UiContainer};

use plygui_cocoa::common;

use self::cocoa::foundation::{NSString, NSRect, NSSize, NSPoint};
use self::cocoa::base::id;
use objc::runtime::{Class, Object, Sel};
use objc::declare::ClassDecl;

use std::mem;
use std::cmp::max;
use std::os::raw::{c_void, c_int};

lazy_static! {
	static ref WINDOW_CLASS: common::RefClass = unsafe { register_window_class() };
}

const BASE_CLASS: &str = "Scintilla";

#[repr(C)]
pub struct Scintilla {
    base: common::CocoaControlBase,
    
    fn_ptr: Option<extern "C" fn(*mut c_void, c_int, c_int, c_int)>,
    self_ptr: Option<*mut c_void>,
}

impl Scintilla {
	pub fn new() -> Box<Scintilla> {
		Box::new(Scintilla {
                     base: common::CocoaControlBase::with_params(
		                     	invalidate_impl,
                             	development::UiMemberFunctions {
		                             fn_member_id: member_id,
								     fn_is_control: is_control,
								     fn_is_control_mut: is_control_mut,
								     fn_size: size,
	                             },
                             ),
                     fn_ptr: None,
				     self_ptr: None,
                 })
	}
}

impl UiScintilla for Scintilla {
	fn set_margin_width(&mut self, index: usize, width: isize) {
		if let Some(fn_ptr) = self.fn_ptr {
            (fn_ptr)(self.self_ptr.unwrap(), scintilla_sys::SCI_SETMARGINWIDTHN as i32, index as c_int, width as c_int);
        }
	}
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

    fn set_layout_padding(&mut self, padding: layout::BoundarySizeArgs) {
	    	self.base.control_base.layout.padding = padding.into();
		self.base.invalidate();
    }
    fn set_layout_margin(&mut self, margin: layout::BoundarySizeArgs) {
	    	self.base.control_base.layout.margin = margin.into();
		self.base.invalidate();
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
	fn as_member(&self) -> &UiMember {
		self
	}
	fn as_member_mut(&mut self) -> &mut UiMember {
		self
	}
}
impl development::UiDrawable for Scintilla {
	fn draw(&mut self, coords: Option<(i32, i32)>) {
    	if coords.is_some() {
    		self.base.coords = coords;
    	}
    	if let Some((x, y)) = self.base.coords {
    		let (_,ph) = self.parent().unwrap().as_ref().size();
    		unsafe {
	            let mut frame: NSRect = self.base.frame();
	            frame.size = NSSize::new(self.base.measured_size.0 as f64,
	                                     self.base.measured_size.1 as f64);
	            frame.origin = NSPoint::new(x as f64, (ph as i32 - y - self.base.measured_size.1 as i32) as f64);
	            msg_send![self.base.control, setFrame: frame];
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
impl UiControl for Scintilla {
	fn is_container_mut(&mut self) -> Option<&mut UiContainer> { None }
    fn is_container(&self) -> Option<&UiContainer> { None }

	fn on_added_to_container(&mut self, parent: &UiContainer, x: i32, y: i32) {
    	use plygui_api::development::UiDrawable;
    	
        let (pw, ph) = parent.size();
        let (w, h, _) = self.measure(pw, ph);

        let rect = NSRect::new(NSPoint::new(x as f64, (ph as i32 - y - h as i32) as f64),
                               NSSize::new(w as f64, h as f64));

        unsafe {
        	use scintilla_sys::{SCI_GETDIRECTFUNCTION, SCI_GETDIRECTPOINTER};
        	
	        let base: id = msg_send![WINDOW_CLASS.0, alloc];
	        let base: id = msg_send![base, initWithFrame: rect];
	
	        self.base.coords = Some((x as i32, (ph as i32 - y - h as i32) as i32));
	        self.base.control = msg_send![base, autorelease];

        	(&mut *self.base.control).set_ivar(common::IVAR, self as *mut _ as *mut ::std::os::raw::c_void);
        	
        	let fn_ptr: id = msg_send![self.base.control, iMessage:SCI_GETDIRECTFUNCTION wParam:0 lParam:0];
        	let self_ptr: id = msg_send![self.base.control, iMessage:SCI_GETDIRECTPOINTER wParam:0 lParam:0];
        	
            self.fn_ptr = Some(mem::transmute(fn_ptr));
            self.self_ptr = Some(mem::transmute(self_ptr));
        }
    }
    fn on_removed_from_container(&mut self, _: &UiContainer) {
        unsafe { self.base.on_removed_from_container(); }
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
    fn as_has_layout(&self) -> &UiHasLayout { self }
    fn as_has_layout_mut(&mut self) -> &mut UiHasLayout { self }
}
impl UiMember for Scintilla {
	fn set_visibility(&mut self, visibility: types::Visibility) {
        self.base.set_visibility(visibility);
    }
    fn visibility(&self) -> types::Visibility {
        self.base.visibility()
    }
    fn size(&self) -> (u16, u16) {
        self.base.measured_size
    }
    fn on_resize(&mut self, handler: Option<callbacks::Resize>) {
        self.base.h_resize = handler;
    }
	
    unsafe fn native_id(&self) -> usize {
        self.base.control as usize
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
}
unsafe fn register_window_class() -> common::RefClass {
    let superclass = Class::get(BASE_CLASS).unwrap();
    let mut decl = ClassDecl::new(MEMBER_ID_SCINTILLA, superclass).unwrap();

    decl.add_ivar::<*mut c_void>(common::IVAR);

    common::RefClass(decl.register())
}

impl_invalidate!(Scintilla);
impl_is_control!(Scintilla);
impl_size!(Scintilla);
impl_member_id!(MEMBER_ID_SCINTILLA);