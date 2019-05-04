use super::common::*;
use super::*;

use plygui_qt::common::*;
use scintilla_sys::*;

#[repr(C)]
pub struct ConsoleQt {
    base: QtControlBase<Console, ScintillaEditBase>,
    h_command: (bool, SlotSCNotificationPtr<'static>),
}

impl ConsoleQt {
    pub fn new() -> Self {
        let sc = ConsoleQt {
            base: QtControlBase::with_params(ScintillaEditBase::new(), event_handler),
            h_command: (false, SlotSCNotificationPtr::new(move |_| {})),
        };
        sc.base.widget.signals().notify().connect(&sc.h_command.1);
        sc
    }
    pub fn append_text(&mut self, text: &str) {
        let len = text.len();
        let tptr = text.as_bytes().as_ptr();
        unsafe { self.base.widget.as_mut().send(SCI_APPENDTEXT, len, tptr as isize); }
    }
}

impl HasLayoutInner for ConsoleQt {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for ConsoleQt {
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
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        unsafe {
            use plygui_qt::qt_core::cpp_utils::StaticCast;
            
            let ptr = member as *mut _ as u64;
            let qo: &mut QObject = self.base.widget.static_cast_mut();
            qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.base.dirty = false;
        self.draw(member, control);
    }
    fn on_removed_from_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, _: &controls::Container) {}
}

impl HasNativeIdInner for ConsoleQt {
    type Id = QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.static_cast() as *const QObject as *mut QObject)
    }
}
impl HasVisibilityInner for ConsoleQt {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.set_visibility(value);
        self.base.invalidate()
    }
}
impl HasSizeInner for ConsoleQt {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        self.base.widget.set_fixed_size((width as i32, height as i32));
        true
    }
}
impl MemberInner for ConsoleQt {}

impl Drawable for ConsoleQt {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => 42, // TODO min size
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => 42, // TODO min size
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            }
        };
        self.base.dirty = control.measured != old_size;
        (control.measured.0, control.measured.1, self.base.dirty)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate();
    }
}

fn event_handler(object: &mut QObject, event: &mut QEvent) -> bool {
    unsafe {
        let ptr = object.property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
        if ptr != 0 {
            let sc: &mut Console = mem::transmute(ptr);
            match event.type_() {
                QEventType::Resize => {
                    use plygui_api::controls::HasSize;
                    
                    let (width, height) = sc.size();
                    sc.call_on_size(width, height);
                }
                _ => {}
            }
        }
        false
    }
}
default_impls_as!(Console);
