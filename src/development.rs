use plygui_api::development::*;
	
pub trait ScintillaInner: ControlInner {
	fn new() -> Box<Member<Control<Self>>>;
	fn with_content(content: &str) -> Box<Member<Control<Self>>>;
	fn set_margin_width(&mut self, index: usize, width: isize);
}
impl <T: ScintillaInner + Sized + 'static> super::Scintilla for Member<Control<T>> {
	fn set_margin_width(&mut self, index: usize, width: isize) {
		self.as_inner_mut().as_inner_mut().set_margin_width(index, width)
	}
}
impl <T: ScintillaInner + Sized> super::NewScintilla for Member<Control<T>> {
	fn new() -> Box<super::Scintilla> {
		T::new()
	}
	fn with_content(content: &str) -> Box<super::Scintilla> {
		T::with_content(content)
	}
} 

pub trait ConsoleInner: ControlInner + HasLabelInner {
	fn new(with_command_line: bool) -> Box<Member<Control<Self>>>;
	fn exec(&mut self, command: &str);
}
impl <T: ConsoleInner + Sized> super::Console for Member<Control<T>> {
	fn exec(&mut self, command: &str) {
		self.as_inner_mut().as_inner_mut().exec(command)
	}
}
impl <T: ConsoleInner + Sized> super::NewConsole for Member<Control<T>> {
	fn new(with_command_line: bool) -> Box<super::Console> {
		T::new(with_command_line)
	}
}