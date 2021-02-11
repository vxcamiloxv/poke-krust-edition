extern crate pokerust;

use pokerust::phrases::modules::PhrasesFactory;
use pokerust::phrases::modules::english::English;
use crate::pokerust::phrases::traits::factory::Factory;

fn main() {
    let phrases_factory = PhrasesFactory::new();
    let rest = phrases_factory.get_phrases_instance::<English>(vec!(String::from("test")));
    println!("{:?}", rest);
}
