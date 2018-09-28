use super::common::*;

use plygui_gtk::common::*;

use scintilla_sys::{self, Ptr, /*SCNotification,*/ Scintilla as GtkScintilla, ScintillaExt};

use std::str;

#[repr(C)]
pub struct ConsoleGtk {
    base: GtkControlBase<Console>,
}

impl ConsoleGtk {
    pub fn new() -> Self {
        let sc = GtkScintilla::new();
        let sc = ConsoleGtk {
            base: GtkControlBase::with_gtk_widget(sc.upcast::<Widget>()),
        };
        {
            let sc: Widget = sc.base.widget.clone().into();
            let sc = sc.downcast::<GtkScintilla>().unwrap();
            sc.connect_notify(on_notify);
        }
        sc.base.widget.connect_size_allocate(on_size_allocate);
        let widget: Widget = sc.base.widget.clone().into();
        let widget = widget.downcast::<GtkScintilla>().unwrap();
        widget.send_message(scintilla_sys::SCI_SETCODEPAGE as u32, super::Codepage::Utf8 as isize as u64, 0);
        widget.send_message(scintilla_sys::SCI_SETWRAPMODE as u32, scintilla_sys::SC_WRAP_CHAR as u64, 0);
        sc
    }
    pub fn append_text(&mut self, text: &str) {
        let len = text.len();
        let tptr = text.as_bytes().as_ptr();
        let widget: Widget = self.base.widget.clone().into();
        widget.downcast::<GtkScintilla>().unwrap().send_message(scintilla_sys::SCI_APPENDTEXT as u32, len as u64, tptr as i64);
    }
}

impl HasLayoutInner for ConsoleGtk {
    fn on_layout_changed(&mut self, _: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for ConsoleGtk {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.base.set_pointer(member as *mut _ as *mut c_void);
        self.measure(member, control, pw, ph);
        self.draw(member, control, Some((x, y)));
    }
    fn on_removed_from_container(&mut self, _: &mut MemberBase, _: &mut ControlBase, _: &controls::Container) {}

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

impl MemberInner for ConsoleGtk {
    type Id = GtkWidget;

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

impl Drawable for ConsoleGtk {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase, coords: Option<(i32, i32)>) {
        self.base.draw(member, control, coords);
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        let old_size = self.base.measured_size;
        let (lm, tm, rm, bm) = self.base.margins().into();

        self.base.measured_size = match member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => w,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => {
                        42 as u16 // TODO min_width
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => h,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => {
                        42 as u16 // TODO min_height
                    }
                };
                (cmp::max(0, w as i32 + lm + rm) as u16, cmp::max(0, h as i32 + tm + bm) as u16)
            }
        };
        (self.base.measured_size.0, self.base.measured_size.1, self.base.measured_size != old_size)
    }
    fn invalidate(&mut self, _: &mut MemberBase, _: &mut ControlBase) {
        self.base.invalidate()
    }
}

impl_all_defaults!(Console);

fn on_size_allocate(this: &::plygui_gtk::gtk::Widget, _allo: &::plygui_gtk::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    if let Some(ll) = cast_gtk_widget_to_member_mut::<Console>(&mut ll) {
        let measured_size = ll.as_inner().as_inner().size();
        ll.call_on_resize(measured_size.0 as u16, measured_size.1 as u16);
    }
}

fn on_notify(_this: &GtkScintilla, _msg: i32, _notification: Ptr, _data: Ptr) {
    //let mut b = this.clone().upcast::<Widget>();
    //let notification = unsafe { &*(notification as *const SCNotification) };
}
