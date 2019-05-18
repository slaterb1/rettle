extern crate rettle;

use rettle::pot::Pot;
use rettle::ingredient::{Fill, Steep, Pour};
use rettle::brewer::Brewer;
use rettle::tea::{Tea, RawTea1};

fn main() {
    let mut new_pot = Pot::new();
    new_pot.add_source(Box::new(Fill{
        name: String::from("fake_tea"),
        source: String::from("hardcoded"),
    }));
    new_pot.add_ingredient(Box::new(Steep{
        name: String::from("steep1"),
        computation: Box::new(|tea: &Tea| {
            let x = tea.data.x;
            let x = x - 1234567;
            let new_tea = Tea { data: RawTea1 { x, str_val: String::from(&tea.data.str_val[..]), y: false } };
            new_tea
        }),
    }));
    new_pot.add_ingredient(Box::new(Pour{
        name: String::from("pour1"),
        computation: Box::new(|tea: &Tea| {
            println!("Final Tea: {:?}", tea);
            Tea { data: RawTea1 { x: tea.data.x, str_val: String::from(&tea.data.str_val[..]), y: tea.data.y } }
        }),
    }));
    let new_brewer = Brewer::new();
    //new_brewer.update_steps(new_pot.get_recipe());
    new_pot.brew(new_brewer);
    println!("Number of sources: {}", new_pot.get_sources().len());
    println!("Number of steps: {}", new_pot.get_recipe().len());
}
