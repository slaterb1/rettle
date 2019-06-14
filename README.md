# rettle
This library is a multithreaded ETL (**E**xtract, **T**ransfrom, **L**oad), with inspiration drawn from Keras (https://keras.io/), to allow a "Brew Master" to define any order of operations for data transformations and outputs.

## Types
rettle has the following Types to be used in any project to "Brew" data:
- **Pot:** container that holds the set of instructions for data sources, sinks, and transforms (*See Ingredient Types below*)
- **Brewery:** manager that holds the brewers and sends them jobs and the initial state of tea to be processed
- **Brewer:** worker that brews the Tea

## Traits
- **Tea:** inherited by custom data struct defined that will be transformed in the ETL pipeline
- **Ingredient:** defines the steps that can be included in the ETL recipe
- **Argument:** defines additional params that an Ingredient operation can use (Optional)

## Ingredient Types
- **Fill:** data input source
- **Transfuse:** combine data from multiple sources defined before this step *Not Implemented Yet*
- **Steep:** data transformation step
- **Skim:** remove a field (or Tea object) *Not Implemented Yet*
- **Pour:** data output destination

## Using rettle
In your custom project you first need to define the custom "Tea" struct that will be created by the `Fill` Ingredient.

Example:
```rust
pub struct TextTea {
    pub x: i32,
    pub str_val: String,
    pub y: bool,
}
```

Plus implement the `Tea` trait methods.

Example:
```rust
impl Tea for TextTea {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new(self: Box<Self>) -> Box<dyn Tea> {
        let data = r#"{
          "x": 1,
          "str_val": "new_values",
          "y": false
        }"#;
        let data: TextTea = serde_json::from_str(data).unwrap();
        Box::new(data)
    }
}
```

Next you can create a new `Pot` struct and supply it with sources and ingredients before calling it's `brew()` method to kick off the brewing process. Ingredients can be supplied with Optional `Argument` trait structs to pass additional runtime parameters used by your custom filters. Finally a `Brewery` struct must be created to specify the number of `Brewers` to run the code, and a `start_time` value to provide elapsed run time metrics.

`Fill` operations collect and pass the `Tea` objects to be worked on to the `Brewery` for it to be processed by the `Brewers`.

### Example Project Code
```rust
fn main() {
    let start_time = Instant::now();
    let mut new_pot = Pot::new();
    let brewery = Brewery::new(2, start_time);
    let steep_args = SteepArgs { increment: 10000 };
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
                    tea_batch.push(TextTea::new(Box::new(TextTea::default())));
                }
                let recipe = Arc::clone(&recipe);
                brewery.take_order(|| {
                    make_tea(tea_batch, recipe);
                });
            }
        }),
        params: None,
    }));
    new_pot.add_ingredient(Box::new(Steep{
        name: String::from("steep1"),
        computation: Box::new(|tea_batch, args| {
            tea_batch.into_iter()
                .map(|tea| {
                    let tea = tea.as_any().downcast_ref::<TextTea>().unwrap();
                    let mut new_tea = tea.clone();
                    match args {
                        None => panic!("No params passed, not editing object!"),
                        Some(box_args) => {
                            let box_args = box_args.as_any().downcast_ref::<SteepArgs>().unwrap();
                            new_tea.x = new_tea.x - box_args.increment;
                        }
                    }
                    Box::new(new_tea) as Box<dyn Tea + Send>
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
                    let tea = tea.as_any().downcast_ref::<TextTea>().unwrap();
                    let same_tea = TextTea { x: tea.x, str_val: String::from(&tea.str_val[..]), y: tea.y };
                    Box::new(same_tea) as Box<dyn Tea + Send>
                })
                .collect()
        }),
        params: None,
    }));
    new_pot.brew(&brewery);

    brewery.get_brewer_info();
    println!("Number of sources: {}", new_pot.get_sources().len());
    println!("Number of steps: {}", new_pot.get_recipe().read().unwrap().len());
}
```

## Next Steps
- Investigate data management/organization strategies for storing Intermediate data transformation structs throughout the ETL process
- Look into implementing `par_iter()`
- Further benchmarks for speed processing data as well as comparing against other ETLs (i.e. Logstash, Spark, etc)
