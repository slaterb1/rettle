extern crate rettle;

use rettle::{Pot, Ingredient};

fn main() {
    let mut new_pot = Pot::new();
    new_pot.add(Ingredient::Fill);
    new_pot.add(Ingredient::Steep);
    new_pot.add(Ingredient::Pour);
    let count = new_pot.brew();
    println!("Count: {}", count);
}
