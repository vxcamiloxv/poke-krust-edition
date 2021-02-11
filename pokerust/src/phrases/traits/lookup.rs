use std::boxed::Box;
use std::fmt::*;
use std::any::Any;
use super::greetings::Greetings;

pub trait Lookup {
    fn get_name(&self) -> &String;

    fn create_instance(&self, args: Vec<String>) -> Box<dyn Greetings>;

    fn as_any(&self) -> &dyn Any;
}

impl Debug for Box<dyn Lookup> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Language {}", self.get_name())
    }
}
