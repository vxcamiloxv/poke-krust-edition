pub mod english;
pub mod spanish;

use std::boxed::Box;

use crate::phrases::traits::factory::Factory;
use crate::phrases::traits::greetings::Greetings;
use crate::phrases::traits::lookup::Lookup;
use self::english::English;
use self::spanish::Spanish;

fn get_name_from_type<C>() -> String {
    String::from(std::any::type_name::<C>().split("::").last().unwrap())
}

pub struct PhrasesFactory {
    phrases_list: Vec<Box<dyn Lookup>>
}

impl PhrasesFactory {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut list: Vec<Box<dyn Lookup>> = Vec::new();
        list.push(Box::new(English { name: String::from("english") }));
        list.push(Box::new(Spanish { name: String::from("spanish") }));
        Self { phrases_list: list }
    }
}

impl Factory for PhrasesFactory {
    fn get_phrases_instance<C:'static + Lookup>(&self, args: Vec<String>) -> Box<dyn Greetings> {
        let result = self.phrases_list
            .iter()
            .filter(|&x| x.get_name() == &get_name_from_type::<C>().to_lowercase())
            .nth(0)
            .unwrap()
            .as_any()
            .downcast_ref::<C>()
            .unwrap();

        result.create_instance(args)
    }
}
