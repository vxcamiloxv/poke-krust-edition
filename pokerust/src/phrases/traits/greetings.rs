use std::boxed::Box;
use std::fmt::*;

pub trait Greetings {
    fn hello(&self) -> String;

    fn goodbye(&self) -> String;
}

impl Debug for Box<dyn Greetings> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.hello(), self.goodbye())
    }
}
