use rettle::pot::Pot;
use rettle::ingredient::{Fill, Steep, Skim, Pour, Argument};
use rettle::tea::Tea;
use rettle::brewery::{Brewery, make_tea};

use serde::{Deserialize, Serialize};
use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time::Instant;

// Example object that implements the Tea trait
#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
/// Test struct having the Tea trait created by Fill operation.
pub struct TextTea {
    pub x: Option<i32>,
    pub str_val: Option<String>,
    pub y: Option<bool>,
}

impl Tea for TextTea {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Setup Argument Trait structs that are used in computations.
pub struct FillArgs {
    pub batch_size: usize,
    pub docs_to_create: usize,
}

impl Argument for FillArgs {
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

pub struct PourArgs {
    pub counter: Arc<Mutex<i32>>,
}

impl Argument for PourArgs {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    // Initialize pot, brewery.
    let start_time = Instant::now();
    let mut new_pot = Pot::new();
    let brewery = Brewery::new(2, start_time);

    // Setup example params.
    let fill_args1 = FillArgs { batch_size: 200, docs_to_create: 1000000 };
    let fill_args2 = FillArgs { batch_size: 200, docs_to_create: 100000 };
    let steep_args = SteepArgs { increment: 10000 };
    let counter = Arc::new(Mutex::new(0));
    let pour_args = PourArgs { counter };
    
    // Add sources to pot.
    // source 1:
    new_pot.add_source(Box::new(Fill {
        name: String::from("fake_tea1"),
        source: String::from("hardcoded"),
        computation: Box::new(|args, brewery, recipe| {
            // Extract run vals from params.
            let (batch_size, total_data) = match args {
                None => panic!("Expected args for this example!"),
                Some(box_args) => {
                    let box_args = box_args.as_any().downcast_ref::<FillArgs>().unwrap();
                    let FillArgs { batch_size, docs_to_create } = box_args;
                    (*batch_size, *docs_to_create)
                }
            };

            let num_iterations = total_data / batch_size;
            println!("Testing {} iterations", total_data);

            for _ in 0 .. num_iterations {
                let mut tea_batch = Vec::with_capacity(batch_size);
                for _ in 0 .. batch_size {
                    tea_batch.push(Box::new(TextTea { x: Some(0), str_val: Some(String::new()), y: Some(true) }) as Box<dyn Tea + Send>);
                }
                let recipe = Arc::clone(&recipe);
                brewery.take_order(|| {
                    make_tea(tea_batch, recipe);
                });
            }
        }),
        params: Some(Box::new(fill_args1)),
    }));
    
    // source 2:
    new_pot.add_source(Box::new(Fill{
        name: String::from("fake_tea2"),
        source: String::from("hardcoded"),
        computation: Box::new(|args, brewery, recipe| {
            // Extract run vals from params.
            let (batch_size, total_data) = match args {
                None => panic!("Expected args for this example!"),
                Some(box_args) => {
                    let box_args = box_args.as_any().downcast_ref::<FillArgs>().unwrap();
                    let FillArgs { batch_size, docs_to_create } = box_args;
                    (*batch_size, *docs_to_create)
                }
            };
            
            let num_iterations = total_data / batch_size;
            println!("Testing {} iterations", total_data);

            for _ in 0 .. num_iterations {
                let mut tea_batch = Vec::with_capacity(batch_size);
                for _ in 0 .. batch_size {
                    tea_batch.push(Box::new(TextTea { x: Some(0), str_val: Some(String::new()), y: Some(true) }) as Box<dyn Tea + Send>);
                }
                let recipe = Arc::clone(&recipe);
                brewery.take_order(|| {
                    make_tea(tea_batch, recipe);
                });
            }
        }),
        params: Some(Box::new(fill_args2)),
    }));
    
    // Add ingredients to pot.
    // steep 1:
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
                            tea.x = Some(tea.x.unwrap() - box_args.increment);
                        }
                    }
                    Box::new(tea) as Box<dyn Tea + Send>
                })
                .collect()
        }),
        params: Some(Box::new(steep_args)),
    }));
    
    // skim 1:
    new_pot.add_ingredient(Box::new(Skim{
        name: String::from("skim1"),
        computation: Box::new(|tea_batch, _args| {
            tea_batch
                .into_iter()
                .map(|tea| {
                    let mut tea = tea.as_any().downcast_ref::<TextTea>().unwrap().clone();
                    tea.y = None;
                    Box::new(tea) as Box<dyn Tea + Send>
                })
                .collect()
        }),
        params: None,
    }));
    
    // pour 1:
    new_pot.add_ingredient(Box::new(Pour{
        name: String::from("pour1"),
        computation: Box::new(|tea_batch, args| {
            // Count batches flowing through Pour operation.
            match args {
                None => println!("No params passed"),
                Some(box_args) => {
                    let box_args = box_args.as_any().downcast_ref::<PourArgs>().unwrap();
                    let mut num = box_args.counter.lock().unwrap();
                    *num += 1;
                    println!("Pouring Batch Number:{}", num);
                }
            };

            // Return unchanged tea_batch for future steps
            tea_batch
        }),
        params: Some(Box::new(pour_args)),
    }));
    
    // Process Tea
    new_pot.brew(&brewery);

    // Display information
    brewery.get_brewer_info();
    println!("Number of sources: {}", new_pot.get_sources().len());
    println!("Number of steps: {}", new_pot.get_recipe().read().unwrap().len());

    println!("Expected number of batchs: {}", 1100000 / 200);
}
