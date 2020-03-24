use plygui_api::{
    controls::{Member, HasLabel},
    sdk::{AControl, HasInner, AMember, HasLabelInner, Abstract},
};
use crate::Scintilla;
use crate::sdk::{ScintillaInner, AScintilla};

define! {
    Console: Scintilla + HasLabel {
        outer: {
            fn exec(&mut self, command: &str);
        }
        inner: {
            fn exec(&mut self, command: &str);
        }
        constructor: {
            fn with_path<S: AsRef<str>>(path: S) -> Box<dyn Console>;
        }
    }
}

impl<T: ConsoleInner> Console for AMember<AControl<AScintilla<AConsole<T>>>> {
    fn exec(&mut self, command: &str) {
        self.inner_mut().inner_mut().inner_mut().inner_mut().exec(command)
    }
    fn as_console (& self) -> & dyn Console { self } 
    fn as_console_mut (& mut self) -> & mut dyn Console { self } 
    fn into_console (self : Box < Self >) -> Box < dyn Console >  { self }
}

impl<T: ConsoleInner> NewConsole for AMember<AControl<AScintilla<AConsole<T>>>> {
    fn with_path<S: AsRef<str>>(path: S) -> Box<dyn Console> {
        T::with_path(path)
    }
}
