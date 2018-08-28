use plygui_api::callbacks::Callback;
use plygui_api::development::*;

pub trait ScintillaInner: ControlInner {
    fn new() -> Box<Member<Control<Self>>>;
    fn set_margin_width(&mut self, index: usize, width: isize);
    fn set_readonly(&mut self, readonly: bool);
    fn is_readonly(&self) -> bool;
    fn set_codepage(&mut self, cp: super::Codepage);
    fn codepage(&self) -> super::Codepage;
    fn append_text(&mut self, text: &str);
    fn on_ui_update(&mut self, Option<Custom>);
}
impl<T: ScintillaInner + Sized + 'static> super::Scintilla for Member<Control<T>> {
    fn set_margin_width(&mut self, index: usize, width: isize) {
        self.as_inner_mut().as_inner_mut().set_margin_width(index, width)
    }
    fn set_readonly(&mut self, readonly: bool) {
        self.as_inner_mut().as_inner_mut().set_readonly(readonly)
    }
    fn is_readonly(&self) -> bool {
        self.as_inner().as_inner().is_readonly()
    }
    fn append_text(&mut self, text: &str) {
        self.as_inner_mut().as_inner_mut().append_text(text)
    }
    /*fn set_codepage(&mut self, cp: super::Codepage) {
        self.as_inner_mut().as_inner_mut().set_codepage(cp)
    }
    fn codepage(&self) -> super::Codepage {
        self.as_inner().as_inner().codepage()
    }*/
}
impl<T: ScintillaInner + Sized> super::NewScintilla for Member<Control<T>> {
    fn new() -> Box<super::Scintilla> {
        T::new()
    }
    fn with_content(content: &str) -> Box<super::Scintilla> {
    	use super::Scintilla;
    	
        let mut sc = T::new();
        sc.append_text(content);
        sc
    }
}

pub trait ConsoleInner: ControlInner + HasLabelInner {
    fn new(with_command_line: bool) -> Box<Member<Control<Self>>>;
    fn exec(&mut self, command: &str);
}
impl<T: ConsoleInner + Sized> super::Console for Member<Control<T>> {
    fn exec(&mut self, command: &str) {
        self.as_inner_mut().as_inner_mut().exec(command)
    }
}
impl<T: ConsoleInner + Sized> super::NewConsole for Member<Control<T>> {
    fn new(with_command_line: bool) -> Box<super::Console> {
        T::new(with_command_line)
    }
}

callback!(Custom, FnMut(&mut super::Scintilla));
