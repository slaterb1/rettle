extern crate rettle;

use rettle::pot::Pot;
use rettle::ingredient::{Fill, Steep, Pour};
use rettle::tea::Tea;

use serde::{Deserialize, Serialize};
use std::any::Any;

// Example object that implements the Tea trait
#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// Resulting data that is being manipulated in the brew.
pub struct TextTea {
    pub x: i32,
    pub str_val: String,
    pub y: bool,
}

impl TextTea {
    pub fn new() -> Box<dyn Tea> {
        let data = r#"{
          "x": 1,
          "str_val": "new_values",
          "y": false
        }"#;
        let data: TextTea = serde_json::from_str(data).unwrap();
        Box::new(data)
    }
}

impl Tea for TextTea {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let mut new_pot = Pot::new();
    new_pot.add_source(Box::new(Fill{
        name: String::from("fake_tea"),
        source: String::from("hardcoded"),
        computation: Box::new(|| {
            TextTea::new()
        }),
    }));
    new_pot.add_ingredient(Box::new(Steep{
        name: String::from("steep1"),
        computation: Box::new(|tea: &Box<dyn Tea>| {
            let tea = tea.as_any().downcast_ref::<TextTea>().unwrap();
            let x = tea.x;
            let x = x - 1234567;
            let new_tea = TextTea { x, str_val: String::from(&tea.str_val[..]), y: false };
            Box::new(new_tea)
        }),
    }));
    new_pot.add_ingredient(Box::new(Pour{
        name: String::from("pour1"),
        computation: Box::new(|tea: &Box<dyn Tea>| {
            println!("Final Tea: {:?}", tea.as_any().downcast_ref::<TextTea>().unwrap());
            let tea = tea.as_any().downcast_ref::<TextTea>().unwrap();
            let same_tea = TextTea { x: tea.x, str_val: String::from(&tea.str_val[..]), y: tea.y };
            Box::new(same_tea)
        }),
    }));
    //new_brewer.update_steps(new_pot.get_recipe());
    new_pot.brew();
    println!("Number of sources: {}", new_pot.get_sources().len());
    println!("Number of steps: {}", new_pot.get_recipe().len());
}
