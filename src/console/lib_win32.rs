use super::development as scintilla_dev;

use plygui_api::controls;
use plygui_api::development::*;		
		
use plygui_win32::common;

use winapi::shared::windef;
use winapi::shared::minwindef;
use winapi::um::winuser;
use winapi::um::commctrl;
use winapi::um::wincon;
use winapi::um::consoleapi;

use std::thread;
use std::sync::mpsc;
//use std::os::windows::ffi::OsStrExt;
//use std::ffi::OsStr;

pub type Console = Member<Control<ConsoleWin32>>;

const NO_CONSOLE_NAME: &str = "Plygui Unnamed Console";

enum ConsoleThread {
    Idle(String),
    Running(thread::JoinHandle<()>)
}
enum TxCommand {
    Exit
}
enum RxCommand {
    Error,
}

#[repr(C)]
pub struct ConsoleWin32 {
    scintilla: super::scintilla::lib_win32::ScintillaWin32,
    
    cmd: ConsoleThread,
    tx: Option<mpsc::Sender<TxCommand>>,
    rx_out: mpsc::Receiver<RxCommand>,
    rx_in: mpsc::Sender<RxCommand>,
}

impl scintilla_dev::ConsoleInner for ConsoleWin32 {
	fn new(with_command_line: bool) -> Box<Console> {
		use development::ScintillaInner;
		use plygui_api::development::HasInner;
		
		let (rx_in, rx_out) = mpsc::channel();
		let b: Box<Console> = Box::new(Member::with_inner(Control::with_inner(
        		ConsoleWin32 {
		            scintilla: super::scintilla::lib_win32::ScintillaWin32::new().into_inner().into_inner(),
		            cmd: ConsoleThread::Idle(NO_CONSOLE_NAME.into()),
		            tx: None,
		            rx_in: rx_in,
		            rx_out: rx_out,
		        }, ()),
        		MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        //b.set_layout_padding(layout::BoundarySize::AllTheSame(DEFAULT_PADDING).into());
        b
	}
	fn exec(&mut self, command: &str) {
		
	}
}

impl HasLabelInner for ConsoleWin32 {
    fn label(&self) -> ::std::borrow::Cow<str> {
        match self.cmd {
            ConsoleThread::Idle(ref name) => ::std::borrow::Cow::Borrowed(name),
            ConsoleThread::Running(ref handle) => ::std::borrow::Cow::Borrowed(handle.thread().name().unwrap_or(NO_CONSOLE_NAME))
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        match self.cmd {
            ConsoleThread::Idle(ref mut name) => *name = label.into(),
            ConsoleThread::Running(_) => {} // TODO warn
        }
    }
}

impl ControlInner for ConsoleWin32 {
	fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container, x: i32, y: i32) {
		self.scintilla.on_added_to_container(base, parent, x, y);
		let name = match self.cmd {
    		ConsoleThread::Idle(ref name) => name.clone(),
    		_ => { unreachable!() }
		};
		let (tx_in, tx_out) = mpsc::channel();
		self.tx = Some(tx_in);
		let rx_in = self.rx_in.clone();
		self.cmd = ConsoleThread::Running(thread::Builder::new().name(name).spawn(move ||{
    		unsafe { 
    		    consoleapi::AllocConsole(); 
    		    
    		    loop {
    		        match tx_out.try_recv() {
    		            Ok(cmd) => match cmd {
    		                TxCommand::Exit => break,
    		            },
    		            Err(_) => {
    		                let _ = rx_in.send(RxCommand::Error);
    		                break;
    		            }
    		        }
    		    }
    		}
		}).unwrap());
	}
    fn on_removed_from_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container) {
        let _ = self.tx.as_mut().unwrap().send(TxCommand::Exit);
    	self.scintilla.on_removed_from_container(base, parent);
    }
    
    fn parent(&self) -> Option<&controls::Member> {
		self.scintilla.parent().map(|p| p.as_member())
	}
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
    	self.scintilla.parent_mut().map(|p| p.as_member_mut())
    }
    fn root(&self) -> Option<&controls::Member> {
    	self.scintilla.root().map(|p| p.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
    	self.scintilla.root_mut().map(|p| p.as_member_mut())
    }
    
    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, base: &mut development::MemberControlBase, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
    	fill_from_markup_base!(self, base, markup, registry, Console, ["Console"]);
    }
}

impl Drop for ConsoleWin32 {
    fn drop(&mut self) {
        
    }
}

impl HasLayoutInner for ConsoleWin32 {
	fn on_layout_changed(&mut self, base: &mut MemberBase) {
		self.scintilla.on_layout_changed(base)
	}
}

impl MemberInner for ConsoleWin32 {
	type Id = common::Hwnd;
	
	fn size(&self) -> (u16, u16) {
        let rect = unsafe { common::window_rect(self.scintilla.native_id().into()) };
        (
            (rect.right - rect.left) as u16,
            (rect.bottom - rect.top) as u16,
        )
    }

    fn on_set_visibility(&mut self, base: &mut MemberBase) {
	    self.scintilla.on_set_visibility(base)
    }
    unsafe fn native_id(&self) -> Self::Id {
        self.scintilla.native_id()
    }
}

impl Drawable for ConsoleWin32 {
	fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>) {
		self.scintilla.draw(base, coords);
	}
    fn measure(&mut self, base: &mut MemberControlBase, w: u16, h: u16) -> (u16, u16, bool) {
    	self.scintilla.measure(base, w, h)
    }
    fn invalidate(&mut self, base: &mut MemberControlBase) {
    	self.scintilla.invalidate(base)
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
	use super::NewConsole;
	
    Console::new(false).into_control()
}

impl_all_defaults!(Console);
