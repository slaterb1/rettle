# rettle
This library is an ETL (**E**xtract, **T**ransfrom, **L**oad), with inspiration drawn from Keras (https://keras.io/), to allow a "Brew Master" to define any order of operations for data transformations and outputs.

## Types
rettle has the following Types to be used in any project to "Brew" data:
- **Pot:** container that holds the set of instructions for data sources, sinks, and transforms (*See Ingredient Types below*)
- **Brewer:** worker / channel processing Tea

## Traits
- **Tea:** inherited by custom data struct defined that will be transformed in the ETL pipeline
- **Ingredient:** defines the steps that can be included in the ETL recipe

## Ingredient Types
- **Fill:** data input source
- **Transfuse:** combine data from multiple sources defined before this step
- **Steep:** data transformation step
- **Skim:** remove a field (or Tea object)
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

Next you can create a new `Pot` stuct and supply it with sources and ingredients before calling it's `brew()` method.

```rust
fn main() {
    let mut new_pot = Pot::new();
    new_pot.add_source(Box::new(Fill{
        name: String::from("fake_tea"),
        source: String::from("hardcoded"),
        computation: Box::new(|| {
            TextTea::new(Box::new(TextTea::default()))
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
    new_pot.brew();
    println!("Number of sources: {}", new_pot.get_sources().len());
    println!("Number of steps: {}", new_pot.get_recipe().len());
}
```

## Next Steps
- implement copy trait on `Tea`
- implement concurrency with brewer pool
- implement custom input params to `computation` on structs with `Ingredient` trait
