extern crate rettle;

use rettle::pot::Pot;
use rettle::ingredient::{Fill, Steep, Pour};
pub use rettle::brewer::Brewer;

fn main() {
    let mut new_pot = Pot::new();
    new_pot.add_source(Box::new(Fill{
        name: String::from("fake_tea"),
        source: String::from("hardcoded"),
    }));
    new_pot.add_ingredient(Box::new(Steep{
        name: String::from("steep1")
    }));
    new_pot.add_ingredient(Box::new(Pour{
        name: String::from("pour1")
    }));
    let new_brewer = Brewer::new();
    //new_brewer.update_steps(new_pot.get_recipe());
    new_pot.brew(new_brewer);
    println!("Number of sources: {}", new_pot.get_sources().len());
    println!("Number of steps: {}", new_pot.get_recipe().len());
}
