use crate::development::*;
use plygui_api::*;
use plygui_api::development::*;

pub type Console = AMember<AControl<AScintilla<AConsole<ScintillaConsole>>>>;

use std::sync::mpsc;
use std::{io, process, thread, borrow::Cow};

const NO_CONSOLE_NAME: &str = "Plygui Unnamed Console";

enum ConsoleThread {
    Idle(String),
    Running(thread::JoinHandle<()>, mpsc::Sender<TxCommand>),
}
enum TxCommand {
    Execute(String, Vec<String>),
    Exit,
}
enum RxCommand {
    Ready(Option<i32>),
    Line(String),
    Error,
}

#[repr(C)]
pub struct ScintillaConsole {
    inner: ScintillaControl,
    input: bool,
    cmd: ConsoleThread,
    rx_in: mpsc::Sender<RxCommand>,
    rx_out: mpsc::Receiver<RxCommand>,
}

impl<O: crate::Console> NewConsoleInner<O> for ScintillaConsole {
    fn with_uninit(b: &mut ::std::mem::MaybeUninit<O>) -> Self {
        let (rx_in, rx_out) = mpsc::channel();
        Self {
            inner: <ScintillaControl as NewScintillaInner<O>>::with_uninit(b),
            cmd: ConsoleThread::Idle(NO_CONSOLE_NAME.into()),
            rx_in: rx_in,
            rx_out: rx_out,
            input: false,
        }
    }
}
impl ConsoleInner for ScintillaConsole {
	fn with_path<S: AsRef<str>>(path: S) -> Box<dyn crate::Console> {
	    let mut b: Box<::std::mem::MaybeUninit<Console>> = Box::new_uninit();
        let mut ab = AMember::with_inner(
            AControl::with_inner(
                AScintilla::with_inner(
                    AConsole::with_inner(
                        <Self as NewConsoleInner<Console>>::with_uninit(b.as_mut()),
                    )
                )
            ),
        );
        ab.append_text(path.as_ref());
		unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
	}
	fn exec(&mut self, command: &str) {
        match self.cmd {
            ConsoleThread::Idle(_) => {} // TODO
            ConsoleThread::Running(_, ref mut tx) => {
                let command = command.split(" ").map(|s| s.to_owned()).collect::<Vec<_>>();
                let _ = tx.send(TxCommand::Execute(command[0].clone(), command[1..].into()));
            }
        }
    }
}
impl HasLabelInner for ScintillaConsole {
    fn label(&self,  _: &MemberBase) -> ::std::borrow::Cow<'_, str> {
        match self.cmd {
            ConsoleThread::Idle(ref name) => ::std::borrow::Cow::Borrowed(name),
            ConsoleThread::Running(ref handle, _) => ::std::borrow::Cow::Borrowed(handle.thread().name().unwrap_or(NO_CONSOLE_NAME)),
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        match self.cmd {
            ConsoleThread::Idle(ref mut name) => *name = label.into(),
            ConsoleThread::Running(_, _) => {} // TODO warn
        }
    }
}
impl ScintillaInner for ScintillaConsole {
    fn new() -> Box<dyn crate::Scintilla> {
        Self::with_path("").into_scintilla()	
    }
    fn set_margin_width(&mut self, index: usize, width: isize) {
        self.inner.set_margin_width(index, width)
    }
    fn set_readonly(&mut self, readonly: bool) {
        self.inner.set_readonly(readonly)
    }
    fn is_readonly(&self) -> bool {
        self.inner.is_readonly()
    }
    fn set_codepage(&mut self, cp: crate::Codepage) {
        self.inner.set_codepage(cp)
    }
    fn codepage(&self) -> crate::Codepage {
        self.inner.codepage()
    }
    fn append_text(&mut self, text: &str) {
        self.inner.append_text(text)
    }
}
impl ControlInner for ScintillaConsole {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.inner.on_added_to_container(member, control, parent, x, y, pw, ph);
        {
            let my_id = member.as_member().id();
            #[cfg(all(target_os = "windows", feature = "win32"))]
            let mut app = ::plygui_win32::imp::Application::get().unwrap();
            #[cfg(feature = "gtk3")]
            let mut app = ::plygui_gtk::imp::Application::get().unwrap();
            #[cfg(feature = "qt5")]
            let mut app = ::plygui_qt::imp::Application::get().unwrap();
            #[cfg(all(target_os = "macos", feature = "cocoa_"))]
            let mut app = ::plygui_cocoa::imp::Application::get().unwrap();

            app.on_frame(
                (move |w: &mut dyn (::plygui_api::controls::Application)| {
                    if let Some(console) = w.find_member_mut(types::FindBy::Id(my_id)) {
                        let console = console.as_any_mut().downcast_mut::<Console>().unwrap();
                        match console.inner_mut().inner_mut().inner_mut().inner_mut().rx_out.try_recv() {
                            Ok(cmd) => match cmd {
                                RxCommand::Error => {
                                    println!("RxErr");
                                    false
                                }
                                RxCommand::Line(ref line) => {
                                    console.inner_mut().inner_mut().inner_mut().inner_mut().inner.append_text(line.as_str());
                                    true
                                }
                                RxCommand::Ready(_code) => {
                                    console.inner_mut().inner_mut().inner_mut().inner_mut().inner.append_text("Done\n");
                                    true
                                }
                            },
                            Err(e) => mpsc::TryRecvError::Empty == e,
                        }
                    } else {
                        true
                    }
                })
                .into(),
            );
        }

        let name = match self.cmd {
            ConsoleThread::Idle(ref name) => name.clone(),
            _ => unreachable!(),
        };
        let (tx_in, tx_out) = mpsc::channel();
        let rx_in = self.rx_in.clone();

        self.cmd = ConsoleThread::Running(
            thread::Builder::new()
                .name(name)
                .spawn(move || {
                    //TODO exit/close requested
                    loop {
                        match tx_out.recv() {
                            Ok(cmd) => match cmd {
                                TxCommand::Exit => break,
                                TxCommand::Execute(cmd, args) => {
                                    use std::io::BufRead;

                                    match process::Command::new(cmd).args(args).stdout(process::Stdio::piped()).stderr(process::Stdio::piped()).spawn() {
                                        Ok(mut cmd) => {
                                            let out = io::BufReader::new(cmd.stdout.take().unwrap());
                                            let err = io::BufReader::new(cmd.stderr.take().unwrap());

                                            let rx_in2 = rx_in.clone();
                                            let thread = thread::spawn(move || {
                                                err.lines().for_each(|line| {
                                                    match line {
                                                        Ok(line) => {
                                                            let _ = rx_in2.send(RxCommand::Line(line + "\n"));
                                                        }
                                                        Err(_) => {} //TODO
                                                    }
                                                });
                                            });
                                            let rx_in3 = rx_in.clone();
                                            out.lines().for_each(|line| {
                                                match line {
                                                    Ok(line) => {
                                                        let _ = rx_in3.send(RxCommand::Line(line + "\n"));
                                                    }
                                                    Err(_) => {} //TODO
                                                }
                                            });

                                            thread.join().unwrap();

                                            let status = cmd.wait().unwrap();

                                            let _ = rx_in.send(RxCommand::Ready(status.code()));
                                        }
                                        Err(e) => {
                                            println!("Error creating command: {}", e);
                                            let _ = rx_in.send(RxCommand::Error);
                                        }
                                    }
                                }
                            },
                            Err(_) => {}
                        }
                    }
                })
                .unwrap(),
            tx_in,
        );
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, c: &dyn controls::Container) {
        self.inner.on_removed_from_container(member, control, c)
    }

    fn parent(&self) -> Option<&dyn controls::Member> {
        self.inner.parent()
    }
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.parent_mut()
    }
    fn root(&self) -> Option<&dyn controls::Member> {
        self.inner.root()
    }
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.root_mut()
    }
}
impl HasVisibilityInner for ScintillaConsole {
    fn on_visibility_set(&mut self, base: &mut MemberBase, visibility: types::Visibility) -> bool {
        self.inner.on_visibility_set(base, visibility)
    }
}
impl HasNativeIdInner for ScintillaConsole {
    type Id = <ScintillaControl as HasNativeIdInner>::Id;

    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}
impl HasSizeInner for ScintillaConsole {
    fn on_size_set(&mut self, base: &mut MemberBase, size: (u16, u16)) -> bool {
        self.inner.on_size_set(base, size)
    }
}
impl HasLayoutInner for ScintillaConsole {
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base)
    }
}
impl MemberInner for ScintillaConsole {}
impl Spawnable for ScintillaConsole {
    fn spawn() -> Box<dyn controls::Control> {
        Self::new().into_control()
    }
}
impl Drawable for ScintillaConsole {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.draw(member, control)
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        self.inner.measure(member, control, w, h)
    }
    fn invalidate(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.invalidate(member, control)
    }
}
impl Drop for ScintillaConsole {
	fn drop(&mut self) {
		 match self.cmd {
            ConsoleThread::Idle(_) => {} // TODO
            ConsoleThread::Running(_, ref mut tx) => {
                let _ = tx.send(TxCommand::Exit);
            }
        }
	}
}
