use std::boxed::Box;
use std::any::Any;
use crate::phrases::traits::greetings::Greetings;
use crate::phrases::traits::lookup::Lookup;

#[derive(Debug)]
pub struct Spanish {
    pub name: String
}

impl Greetings for Spanish {
    fn hello(&self) -> String {
        format!("Hola {}!", self.name)
    }

    fn goodbye(&self) -> String {
        format!("Adios {}!", self.name)
    }
}

impl Lookup for Spanish {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn create_instance(&self, args: Vec<String>) -> Box<dyn Greetings> {
        Box::new(Spanish {
            name: args[0].to_string()
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ToString for Spanish {
    fn to_string(&self) -> String {
        return format!("Spanish {}", self.name);
    }
}
