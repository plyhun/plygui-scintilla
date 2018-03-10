use super::*;

use plygui_api::{layout, types, development, callbacks};
use plygui_api::traits::{UiControl, UiHasLayout, UiMember, UiContainer};
use plygui_api::members::MEMBER_ID_BUTTON;

use plygui_qt::common;

use scintilla_sys::ScintillaEditBase;

use qt_core::cpp_utils::{StaticCast, DynamicCast};

use std::mem;
use std::cmp::max;
use std::os::raw::{c_void, c_int};

const DEFAULT_PADDING: i32 = 6;

#[repr(C)]
pub struct Scintilla {
    base: common::QtControlBase,

    fn_ptr: Option<extern "C" fn(*mut c_void, c_int, c_int, c_int)>,
    self_ptr: Option<*mut c_void>,
}

impl Scintilla {
    pub fn new() -> Box<Scintilla> {
        let mut sc = Box::new(Scintilla {
                     base: common::QtControlBase::with_params(
		                     	unsafe { (&mut *ScintillaEditBase::new().into_raw()).static_cast_mut() as &mut common::QWidget},
		                     	invalidate_impl,
                             	development::UiMemberFunctions {
		                             fn_member_id: member_id,
								     fn_is_control: is_control,
								     fn_is_control_mut: is_control_mut,
								     fn_size: size,
	                            },
                             	event_handler,
                             ),
                     fn_ptr: None,
				     self_ptr: None,
                 });
        unsafe {
        	let ptr = sc.as_ref() as *const _ as u64;
        	let qo: &mut common::QObject = sc.base.widget.static_cast_mut();
        	qo.set_property(common::PROPERTY.as_ptr() as *const i8, &common::QVariant::new0(ptr));
        }
        sc
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
    	
    	unsafe {
            let qo: *mut ScintillaEditBase = self.base.widget.dynamic_cast_mut().unwrap();
        	self.fn_ptr = Some(mem::transmute(qo.as_mut().unwrap().send(scintilla_sys::SCI_GETDIRECTFUNCTION, 0, 0)));
            self.self_ptr = Some(qo.as_mut().unwrap().send(scintilla_sys::SCI_GETDIRECTPOINTER, 0, 0) as *mut c_void);
        }
    	
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
        self.base.widget.win_id() as usize
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
	        self.base.widget.as_mut().move_((coords.0 as i32 + lm, coords.1 as i32 + tm));
			self.base.widget.as_mut().set_fixed_size(
				(self.base.measured_size.0 as i32 - lm - rm, self.base.measured_size.1 as i32 - rm - bm)
			);
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
impl_member_id!(MEMBER_ID_BUTTON);

fn event_handler(object: &mut common::QObject, event: &common::QEvent) -> bool {
	unsafe {
		match event.type_() {
			common::QEventType::Resize => {
				let ptr = object as *mut common::QObject;
				if let Some(button) = common::cast_qobject_to_uimember_mut::<Scintilla>(object) {
					let (width,height) = button.size();
					if let Some(ref mut cb) = button.base.h_resize {
		                let w2: &mut Scintilla = ::std::mem::transmute(ptr);
		                (cb.as_mut())(w2, width, height);
		            }
				}
			},
			_ => {},
		} 
		false
	}
}