extern crate rettle;
//extern crate crossbeam_utils;

use rettle::pot::Pot;
use rettle::ingredient::{Fill, Steep, Pour, Argument, Ingredient};
use rettle::tea::Tea;
use rettle::brewer::{Brewery, make_tea};

use serde::{Deserialize, Serialize};
use std::any::Any;
use std::sync::Arc;
//use crossbeam_utils::sync::WaitGroup;

// Example object that implements the Tea trait
#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
/// Resulting data that is being manipulated in the brew.
pub struct TextTea {
    pub x: i32,
    pub str_val: String,
    pub y: bool,
}

impl Tea for TextTea {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new(self: Box<Self>) -> Box<dyn Tea + Send> {
        let data = r#"{
          "x": 1,
          "str_val": "new_values",
          "y": false
        }"#;
        let data: TextTea = serde_json::from_str(data).unwrap();
        Box::new(data)
    }
}

pub struct SteepArgs {
    pub increment: i32,
}

impl Argument for SteepArgs {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    //let wg = WaitGroup::new();
    let mut new_pot = Pot::new();
    let brewery = Brewery::new(4);
    let steep_args = SteepArgs { increment: 10000 };
    new_pot.add_source(Box::new(Fill{
        name: String::from("fake_tea"),
        source: String::from("hardcoded"),
        computation: Box::new(|_args, brewery, recipe| {
            for _ in 0 .. 10 {
                //let wg = wg.clone();
                let tea = TextTea::new(Box::new(TextTea::default()));
                brewery.take_order(move || {
                    make_tea(tea, recipe);
                    //drop(wg);
                });
            }
        }),
        params: None,
    }));
    new_pot.add_ingredient(Box::new(Steep{
        name: String::from("steep1"),
        computation: Box::new(|tea, args| {
            let tea = tea.as_any().downcast_ref::<TextTea>().unwrap();
            let mut new_tea = tea.clone();
            // Access params if they exist, optionally User may take other actions in the None arm
            // if panicking is not desired. Alternatively, box_args can have further match
            // statements for additional optional fields
            match args {
                None => panic!("No params passed, not editing object!"),
                Some(box_args) => {
                    let box_args = box_args.as_any().downcast_ref::<SteepArgs>().unwrap();
                    new_tea.x = tea.x - box_args.increment;
                }
            }
            Box::new(new_tea)
        }),
        params: Some(Box::new(steep_args)),
        //params: None,
    }));
    new_pot.add_ingredient(Box::new(Pour{
        name: String::from("pour1"),
        computation: Box::new(|tea, _args| {
            println!("Final Tea: {:?}", tea.as_any().downcast_ref::<TextTea>().unwrap());
            let tea = tea.as_any().downcast_ref::<TextTea>().unwrap();
            let same_tea = TextTea { x: tea.x, str_val: String::from(&tea.str_val[..]), y: tea.y };
            Box::new(same_tea)
        }),
        params: None,
    }));
    //new_brewer.update_steps(new_pot.get_recipe());
    new_pot.brew(&brewery);
    //wg.wait();
    println!("Number of sources: {}", new_pot.get_sources().len());
    println!("Number of steps: {}", new_pot.get_recipe().lock().unwrap().len());
}
