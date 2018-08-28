use super::development as scintilla_dev;

#[cfg(all(target_os = "macos", feature = "cocoa_"))]
use plygui_cocoa::common::*;
#[cfg(feature = "gtk3")]
use plygui_gtk::common::*;
#[cfg(feature = "qt5")]
use plygui_qt::common::*;
#[cfg(all(target_os = "windows", feature = "win32"))]
use plygui_win32::common::*;

#[cfg(all(target_os = "windows", feature = "win32"))]
type ScintillaNative = super::scintilla::lib_win32::ScintillaWin32;
#[cfg(all(target_os = "macos", feature = "cocoa_"))]
type ScintillaNative = super::scintilla::lib_cocoa::ScintillaCocoa;
#[cfg(feature = "qt5")]
type ScintillaNative = super::scintilla::lib_qt::ScintillaQt;
#[cfg(feature = "gtk3")]
type ScintillaNative = super::scintilla::lib_gtk::ScintillaGtk;

use std::sync::mpsc;
use std::{io, process, thread};
//use std::os::windows::ffi::OsStrExt;
//use std::ffi::OsStr;

pub type Console = Member<Control<ConsoleWin32>>;

const NO_CONSOLE_NAME: &str = "Plygui Unnamed Console";

enum ConsoleThread {
    Idle(String),
    Running((thread::JoinHandle<()>), mpsc::Sender<TxCommand>),
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
pub struct ConsoleWin32 {
    scintilla: ScintillaNative,

    input: bool,
    cmd: ConsoleThread,
    rx_in: mpsc::Sender<RxCommand>,
}

impl scintilla_dev::ConsoleInner for ConsoleWin32 {
    fn new(with_command_line: bool) -> Box<Console> {
        use development::ScintillaInner;
        use plygui_api::development::HasInner;

        let (rx_in, rx_out) = mpsc::channel();
        let mut b: Box<Console> = Box::new(Member::with_inner(
            Control::with_inner(
                ConsoleWin32 {
                    scintilla: ScintillaNative::new().into_inner().into_inner(),
                    cmd: ConsoleThread::Idle(NO_CONSOLE_NAME.into()),
                    rx_in: rx_in,
                    input: with_command_line,
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        b.as_inner_mut().as_inner_mut().scintilla.on_ui_update(Some(
            (move |sc: &mut super::Scintilla| match rx_out.try_recv() {
                Ok(cmd) => match cmd {
                    RxCommand::Error => {}
                    RxCommand::Line(ref line) => sc.append_text(line.as_str()),
                    RxCommand::Ready(_code) => sc.append_text("Done\n"),
                },
                Err(_) => {}
            }).into(),
        ));
        b
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

impl HasLabelInner for ConsoleWin32 {
    fn label(&self) -> ::std::borrow::Cow<str> {
        match self.cmd {
            ConsoleThread::Idle(ref name) => ::std::borrow::Cow::Borrowed(name),
            ConsoleThread::Running(ref handle, _) => ::std::borrow::Cow::Borrowed(handle.thread().name().unwrap_or(NO_CONSOLE_NAME)),
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        match self.cmd {
            ConsoleThread::Idle(ref mut name) => *name = label.into(),
            ConsoleThread::Running(_, _) => {} // TODO warn
        }
    }
}

impl ControlInner for ConsoleWin32 {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.scintilla.on_added_to_container(member, control, parent, x, y, pw, ph);
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

                                    println!("{} {:?}", cmd, args);

                                    match process::Command::new(cmd).args(args).stdout(process::Stdio::piped()).stderr(process::Stdio::piped()).spawn() {
                                        Ok(mut cmd) => {
                                            let out = io::BufReader::new(cmd.stdout.take().unwrap());
                                            let err = io::BufReader::new(cmd.stderr.take().unwrap());

                                            let rx_in2 = rx_in.clone();
                                            let thread = thread::spawn(move || {
                                                err.lines().for_each(|line| {
                                                    let _ = rx_in2.send(RxCommand::Line(line.unwrap() + "\n"));
                                                });
                                            });
                                            let rx_in3 = rx_in.clone();
                                            out.lines().for_each(|line| {
                                                let _ = rx_in3.send(RxCommand::Line(line.unwrap() + "\n"));
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
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &controls::Container) {
        let name = match self.cmd {
            ConsoleThread::Idle(_) => unreachable!(),
            ConsoleThread::Running(ref handle, ref tx) => {
                let _ = tx.send(TxCommand::Exit);
                handle.thread().name().unwrap_or(NO_CONSOLE_NAME).to_owned()
            }
        };
        self.cmd = ConsoleThread::Idle(name);
        self.scintilla.on_removed_from_container(member, control, parent);
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

impl HasLayoutInner for ConsoleWin32 {
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.scintilla.on_layout_changed(base)
    }
}

impl MemberInner for ConsoleWin32 {
    type Id = Hwnd;

    fn size(&self) -> (u16, u16) {
        self.scintilla.size()
    }

    fn on_set_visibility(&mut self, base: &mut MemberBase) {
        self.scintilla.on_set_visibility(base)
    }
    unsafe fn native_id(&self) -> Self::Id {
        self.scintilla.native_id()
    }
}

impl Drawable for ConsoleWin32 {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase, coords: Option<(i32, i32)>) {
        self.scintilla.draw(member, control, coords);
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        self.scintilla.measure(member, control, w, h)
    }
    fn invalidate(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.scintilla.invalidate(member, control)
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
    use super::NewConsole;

    Console::new(false).into_control()
}

impl_all_defaults!(Console);
