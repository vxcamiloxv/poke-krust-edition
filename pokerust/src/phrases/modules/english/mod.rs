use std::boxed::Box;
use std::fmt::*;
use std::any::Any;
use crate::phrases::traits::greetings::Greetings;
use crate::phrases::traits::lookup::Lookup;

#[derive(Debug)]
pub struct English {
    pub name: String
}

impl Greetings for English {
    fn hello(&self) -> String {
        format!("Hello {}!", self.name)
    }

    fn goodbye(&self) -> String {
        format!("Goodbye {}!", self.name)
    }
}

impl Lookup for English {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn create_instance(&self, args: Vec<String>) -> Box<dyn Greetings> {
        Box::new(English {
            name: args[0].to_string()
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ToString for English {
    fn to_string(&self) -> String {
        return format!("English {}", self.name);
    }
}
