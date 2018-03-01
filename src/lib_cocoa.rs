use super::*;

use plygui_api::{layout, types, development, callbacks};
use plygui_api::traits::{UiControl, UiHasLayout, UiMember, UiContainer};

use plygui_cocoa::common;

use self::cocoa::appkit::{NSBezelStyle, NSButton};
use self::cocoa::foundation::{NSString, NSRect, NSSize, NSPoint};
use self::cocoa::base::id;
use objc::runtime::{Class, Object, Sel};
use objc::declare::ClassDecl;

use std::mem;
use std::os::raw::c_void;
use std::borrow::Cow;

lazy_static! {
	static ref WINDOW_CLASS: common::RefClass = unsafe { register_window_class() };
}

const BASE_CLASS: &str = "Scintilla";

#[repr(C)]
pub struct Scintilla {
    base: common::CocoaControlBase,
}

unsafe fn register_window_class() -> common::RefClass {
    let superclass = Class::get(BASE_CLASS).unwrap();
    let mut decl = ClassDecl::new(MEMBER_ID_SCINTILLA, superclass).unwrap();

    decl.add_ivar::<*mut c_void>(common::IVAR);

    common::RefClass(decl.register())
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
                 })
	}
}

impl UiScintilla for Scintilla {
	fn set_margin_width(&mut self, index: usize, width: isize) {
		if self.base.control != 0 {
			msg_send!(self.base.control, message:scintilla_sys::SCI_SETMARGINWIDTHN wParam:index lParam:width);
		}
	}
}

impl_invalidate!(Scintilla);
impl_is_control!(Scintilla);
impl_size!(Scintilla);
impl_member_id!(MEMBER_ID_SCINTILLA);