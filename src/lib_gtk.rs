use super::*;

use plygui_api::{layout, types, development, callbacks};
use plygui_api::traits::{UiControl, UiHasLayout, UiMember, UiContainer};

use plygui_gtk::common;

use scintilla_sys::{Scintilla as GtkScintilla, ScintillaExt, SCNotification, Ptr};

use gtk::{Cast, Widget, WidgetExt, Fixed, FixedExt, Rectangle};

use std::mem;
use std::cmp::max;
use std::os::raw::{c_void, c_int};

#[repr(C)]
pub struct Scintilla {
    base: common::GtkControlBase,

    fn_ptr: Option<extern "C" fn(*mut c_void, c_int, c_int, c_int)>,
    self_ptr: Option<*mut c_void>,
}

impl Scintilla {
    pub fn new() -> Box<Scintilla> {
    	let sc = GtkScintilla::new();
    	let (fn_ptr, self_ptr) = {
            let self_ptr = sc.send_message(scintilla_sys::SCI_GETDIRECTPOINTER, 0, 0);
        	let fn_ptr = sc.send_message(scintilla_sys::SCI_GETDIRECTFUNCTION, 0, 0);
            (fn_ptr, self_ptr)
        };        
    	let mut sc = Box::new(Scintilla {
                     base: common::GtkControlBase::with_params(
		                     	sc.upcast::<Widget>(),
		                     	invalidate_impl,
                             	development::UiMemberFunctions {
		                             fn_member_id: member_id,
								     fn_is_control: is_control,
								     fn_is_control_mut: is_control_mut,
								     fn_size: size,
	                            },
                             ),
                     fn_ptr: Some(unsafe { mem::transmute(fn_ptr) }),
				     self_ptr: Some(self_ptr as *mut c_void),
                 });
        {
        	let ptr = sc.as_ref() as *const _ as *mut std::os::raw::c_void;
        	sc.base.set_pointer(ptr);
        }
        {
        	let sc = sc.base.widget.clone().downcast::<GtkScintilla>().unwrap();
			sc.connect_notify(on_notify);
        }
        sc.base.widget.connect_size_allocate(on_resize_move);
        sc
    }
}

impl UiScintilla for Scintilla {
    fn set_margin_width(&mut self, index: usize, width: isize) {
    	if let Some(fn_ptr) = self.fn_ptr {
            (fn_ptr)(self.self_ptr.unwrap(), scintilla_sys::SCI_SETMARGINWIDTHN as i32, index as c_int, width as c_int);
        }
        
        /*unsafe { 
        	let qo: *mut ScintillaEditBase = self.base.widget.static_cast_mut();
			let _ = qo.as_mut().unwrap().send(scintilla_sys::SCI_SETMARGINWIDTHN as u32, index as c_uint, width as c_int);
        }	*/
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

impl UiControl for Scintilla {
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
    fn on_added_to_container(&mut self, parent: &UiContainer, x: i32, y: i32) {
    	use plygui_api::development::UiDrawable;
    	
        let (pw, ph) = parent.draw_area_size();
        self.measure(pw, ph);
        self.draw(Some((x, y)));
    }
    fn on_removed_from_container(&mut self, _: &UiContainer) {}	
    
    fn as_has_layout(&self) -> &UiHasLayout {
    	self
    }
	fn as_has_layout_mut(&mut self) -> &mut UiHasLayout {
		self
	}
}

impl UiMember for Scintilla {
    fn set_visibility(&mut self, visibility: types::Visibility) {
        self.base.set_visibility(visibility);
        self.base.invalidate();
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
        self.base.pointer() as usize
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

impl development::UiDrawable for Scintilla {
	fn draw(&mut self, coords: Option<(i32, i32)>) {
    	if coords.is_some() {
    		self.base.coords = coords;
    	}
    	if let Some(coords) = self.base.coords {
			let (lm,tm,rm,bm) = self.base.control_base.layout.margin.into();
	        self.base.widget.get_parent().unwrap().downcast::<Fixed>().unwrap().move_(&self.base.widget, coords.0 as i32 + lm, coords.1 as i32 + tm);
			self.base.widget.set_size_request(self.base.measured_size.0 as i32 - lm - rm, self.base.measured_size.1 as i32 - rm - bm);
			if let types::Visibility::Visible = self.base.control_base.member_base.visibility {
				self.base.widget.show();
			} else {
				self.base.widget.hide();
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

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<UiControl> {
	Scintilla::new()
}

impl_invalidate!(Scintilla);
impl_is_control!(Scintilla);
impl_size!(Scintilla);
impl_member_id!(MEMBER_ID_SCINTILLA);

fn on_resize_move(this: &Widget, allo: &Rectangle) {
	let mut b = this.clone().upcast::<Widget>();
	let b = common::cast_gtk_widget_to_uimember_mut::<Scintilla>(&mut b).unwrap();
	if b.base.measured_size.0 as i32 != allo.width || b.base.measured_size.1 as i32 != allo.height {
		use std::cmp::max;
		
		b.base.measured_size = (max(0, allo.width) as u16, max(0, allo.height) as u16);
		if let Some(ref mut cb) = b.base.h_resize {
            let mut w2 = this.clone().upcast::<Widget>();
			let mut w2 = common::cast_gtk_widget_to_uimember_mut::<Scintilla>(&mut w2).unwrap();
			(cb.as_mut())(w2, b.base.measured_size.0 as u16, b.base.measured_size.1 as u16);
        }
	}
}
fn on_notify(this: &GtkScintilla, msg: i32, notification: Ptr, data: Ptr) {
	let mut b = this.clone().upcast::<Widget>();
	let notification = notification as *const SCNotification;
	let b = common::cast_gtk_widget_to_uimember_mut::<Scintilla>(&mut b).unwrap();
	
	println!("AAA");
}
