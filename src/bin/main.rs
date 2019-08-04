use rettle::pot::Pot;
use rettle::ingredient::{Fill, Steep, Pour, Argument};
use rettle::tea::Tea;
use rettle::brewer::{Brewery, make_tea};

use serde::{Deserialize, Serialize};
use std::any::Any;
use std::sync::Arc;
use std::time::Instant;

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
    // Initialize variables
    let start_time = Instant::now();
    let mut new_pot = Pot::new();
    let brewery = Brewery::new(2, start_time);
    let steep_args = SteepArgs { increment: 10000 };
    
    // Add sources to pot
    new_pot.add_source(Box::new(Fill{
        name: String::from("fake_tea"),
        source: String::from("hardcoded"),
        computation: Box::new(|_args, brewery, recipe| {
            let total_data = 1000000;
            let batch_size = 200;
            let num_iterations = total_data / batch_size;
            println!("Testing {} iterations", total_data);
            for _ in 0 .. num_iterations {
                let mut tea_batch = Vec::with_capacity(batch_size);
                for _ in 0 .. batch_size {
                    tea_batch.push(Box::new(TextTea::default()) as Box<dyn Tea + Send>);
                }
                let recipe = Arc::clone(&recipe);
                brewery.take_order(|| {
                    make_tea(tea_batch, recipe);
                });
            }
        }),
        params: None,
    }));
    new_pot.add_source(Box::new(Fill{
        name: String::from("fake_tea2"),
        source: String::from("hardcoded"),
        computation: Box::new(|_args, brewery, recipe| {
            let total_data = 100000;
            let batch_size = 200;
            let num_iterations = total_data / batch_size;
            println!("Testing {} iterations", total_data);
            for _ in 0 .. num_iterations {
                let mut tea_batch = Vec::with_capacity(batch_size);
                for _ in 0 .. batch_size {
                    tea_batch.push(Box::new(TextTea::default()) as Box<dyn Tea + Send>);
                }
                let recipe = Arc::clone(&recipe);
                brewery.take_order(|| {
                    make_tea(tea_batch, recipe);
                });
            }
        }),
        params: None,
    }));
    
    // Add ingredients to pot
    new_pot.add_ingredient(Box::new(Steep{
        name: String::from("steep1"),
        computation: Box::new(|tea_batch, args| {
            tea_batch
                .into_iter()
                .map(|tea| {
                    let mut tea = tea.as_any().downcast_ref::<TextTea>().unwrap().clone();
                    match args {
                        None => panic!("No params passed, not editing object!"),
                        Some(box_args) => {
                            let box_args = box_args.as_any().downcast_ref::<SteepArgs>().unwrap();
                            tea.x = tea.x - box_args.increment;
                        }
                    }
                    Box::new(tea) as Box<dyn Tea + Send>
                })
                .collect()
        }),
        params: Some(Box::new(steep_args)),
    }));
    new_pot.add_ingredient(Box::new(Pour{
        name: String::from("pour1"),
        computation: Box::new(|tea_batch, _args| {
            tea_batch.into_iter()
                .map(|tea| {
                    //println!("Final Tea: {:?}", tea.as_any().downcast_ref::<TextTea>().unwrap());
                    tea
                })
                .collect()
        }),
        params: None,
    }));
    
    // Process Tea
    new_pot.brew(&brewery);

    // Display information
    brewery.get_brewer_info();
    println!("Number of sources: {}", new_pot.get_sources().len());
    println!("Number of steps: {}", new_pot.get_recipe().read().unwrap().len());
}
