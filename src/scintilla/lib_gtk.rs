use super::*;

use plygui_api::development::*;
use plygui_api::{controls, layout, types};

use plygui_gtk::common;

use scintilla_sys::{Ptr, SCNotification, Scintilla as GtkScintilla, ScintillaExt};

use gtk::{Cast, Widget, WidgetExt};

use std::cmp::max;
use std::os::raw::{c_int, c_void};
use std::{mem, str};

pub type Scintilla = Member<Control<ScintillaGtk>>;

#[repr(C)]
pub struct ScintillaGtk {
    base: common::GtkControlBase<Scintilla>,

    fn_ptr: Option<extern "C" fn(*mut c_void, c_int, c_int, c_int)>,
    self_ptr: Option<*mut c_void>,
}

impl super::development::ScintillaInner for ScintillaGtk {
    fn new() -> Box<super::Scintilla> {
        let sc = GtkScintilla::new();
        let (fn_ptr, self_ptr) = {
            let self_ptr = sc.send_message(scintilla_sys::SCI_GETDIRECTPOINTER, 0, 0);
            let fn_ptr = sc.send_message(scintilla_sys::SCI_GETDIRECTFUNCTION, 0, 0);
            (fn_ptr, self_ptr)
        };
        let mut sc = Box::new(Member::with_inner(
            Control::with_inner(
                ScintillaGtk {
                    base: common::GtkControlBase::with_gtk_widget(sc.upcast::<Widget>()),
                    fn_ptr: Some(unsafe { mem::transmute(fn_ptr) }),
                    self_ptr: Some(self_ptr as *mut c_void),
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));

        {
            let ptr = sc.as_ref() as *const _ as *mut std::os::raw::c_void;
            sc.as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        {
            let sc: gtk::Widget = sc.as_inner_mut().as_inner_mut().base.widget.clone().into();
            let sc = sc.downcast::<GtkScintilla>().unwrap();
            sc.connect_notify(on_notify);
        }
        sc.as_inner_mut().as_inner_mut().base.widget.connect_size_allocate(on_size_allocate);
        sc
    }
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

impl HasLayoutInner for ScintillaGtk {
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for ScintillaGtk {
    fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container, x: i32, y: i32) {
        let (pw, ph) = parent.draw_area_size();
        self.measure(base, pw, ph);
        self.base.dirty = false;
        self.draw(base, Some((x, y)));
    }
    fn on_removed_from_container(&mut self, _: &mut MemberControlBase, _: &controls::Container) {}

    fn parent(&self) -> Option<&controls::Member> {
        self.base.parent().map(|m| m.as_member())
    }
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.parent_mut().map(|m| m.as_member_mut())
    }
    fn root(&self) -> Option<&controls::Member> {
        self.base.root().map(|m| m.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.root_mut().map(|m| m.as_member_mut())
    }
}

impl MemberInner for ScintillaGtk {
    type Id = common::GtkWidget;

    fn size(&self) -> (u16, u16) {
        self.base.measured_size
    }

    fn on_set_visibility(&mut self, _: &mut MemberBase) {
        self.base.invalidate()
    }

    unsafe fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl Drawable for ScintillaGtk {
    fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>) {
        self.base.draw(base, coords);
    }
    fn measure(&mut self, base: &mut MemberControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        let old_size = self.base.measured_size;
        let (lp, tp, rp, bp) = base.control.layout.padding.into();
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
                (max(0, w as i32 + lm + rm + lp + rp) as u16, max(0, h as i32 + tm + bm + tp + bp) as u16)
            }
        };
        self.base.dirty = self.base.measured_size != old_size;
        (self.base.measured_size.0, self.base.measured_size.1, self.base.dirty)
    }
    fn invalidate(&mut self, _: &mut MemberControlBase) {
        self.base.invalidate()
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
    Scintilla::new().into_control()
}

impl_all_defaults!(Scintilla);

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Scintilla>(&mut ll).unwrap();

    //if ll.as_inner().as_inner().base.dirty {
    ll.as_inner_mut().as_inner_mut().base.dirty = false;
    let measured_size = ll.as_inner().as_inner().base.measured_size;
    if let Some(ref mut cb) = ll.base_mut().handler_resize {
        let mut w2 = this.clone().upcast::<Widget>();
        let mut w2 = common::cast_gtk_widget_to_member_mut::<Scintilla>(&mut w2).unwrap();
        (cb.as_mut())(w2, measured_size.0 as u16, measured_size.1 as u16);
    }
    //}
}

fn on_notify(this: &GtkScintilla, _msg: i32, notification: Ptr, _data: Ptr) {
    let mut b = this.clone().upcast::<Widget>();
    let notification = unsafe { &*(notification as *const SCNotification) };
    let b = common::cast_gtk_widget_to_member_mut::<Scintilla>(&mut b).unwrap();

    println!("AAA {:?}/{:?} = {}", notification.wParam, notification.lParam, unsafe {
        str::from_utf8_unchecked(::std::slice::from_raw_parts(notification.text as *const u8, notification.length as usize))
    });
}
