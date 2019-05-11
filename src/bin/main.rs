extern crate rettle;

use rettle::{Pot, Fill, Steep, Pour, Brewer};

fn main() {
    let mut new_pot = Pot::new();
    new_pot.add_source(Box::new(Fill{
        name: String::from("dummy_txt"),
        source: String::from("hardcoded"),
    }));
    new_pot.add_ingredient(Box::new(Steep{
        name: String::from("steep1")
    }));
    new_pot.add_ingredient(Box::new(Pour{
        name: String::from("pour1")
    }));
    let new_brewer = Brewer::new(new_pot.get_recipe());
    //new_brewer.update_steps(new_pot.get_recipe());
    let count = new_pot.brew(new_brewer);
    println!("Count: {}", count);
}
