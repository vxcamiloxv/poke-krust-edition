use super::greetings::Greetings;
use crate::phrases::traits::lookup::Lookup;

pub trait Factory {
    fn get_phrases_instance<C:'static + Lookup>(&self, args: Vec<String>) -> Box<dyn Greetings>;
}
