use serde::{Deserialize, Serialize};
use std::any::Any;
//use std::fmt;
//use serde_json::Result;

/// Data Structure that holds the recipe to brew tea (ETL data).
pub struct Pot<'a> {
    recipe:  Vec<Box<dyn Ingredient<'a>>>,
    sources: Vec<Box<dyn Source>>,
}

/// Worker that runs the recipe and brew tea.
// TODO: decouple steps and recipe
pub struct Brewer {
    tea: Tea,
}

// TODO: implement Debug for Box<dyn Ingredient>
// impl<'a> fmt::Debug for Brewer<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, 
//                "Brewer {{ steps: {:?}, tea: {:?} }}", 
//                self.steps.iter().map(|step| &*step), 
//                self.tea
//                )
//     } 
// }

impl<'a> Pot<'a> {
    ///
    /// Initializes Pot with an empty recipe.
    pub fn new() -> Pot<'a> {
        Pot { recipe: Vec::new(), sources: Vec::new() }
    }

    ///
    /// The ingredient is the instruction being added to the brew.
    pub fn add_ingredient(&mut self, ingredient: Box<dyn Ingredient<'a>>) {
        &self.recipe.push(ingredient);
    }

    pub fn add_source(&mut self, source: Box<dyn Source>) {
        &self.sources.push(source);
    }

    pub fn get_recipe(&self) -> &Vec<Box<dyn Ingredient<'a>>> {
        &self.recipe
    }

    ///
    /// This runs the recipe to transform data.
    pub fn brew(&self, mut brewer: Brewer) -> u32 {
        let init_tea = brewer.get_tea();
        println!("Initial tea: {:?}", init_tea);
        for source in self.sources.iter() {
            let fill = source.as_any().downcast_ref::<Fill>().unwrap();
            let tea = fill.collect();
            brewer.update_brew(tea);
            println!("Tea from source: {:?}", brewer.get_tea());
        }
        
        for step in self.recipe.iter() {
            if let Some(steep) = step.as_any().downcast_ref::<Steep>() {
                println!("Steep operation!");
                let tea = steep.exec(brewer.get_tea());
                brewer.update_brew(tea);
                println!("Tea after steep: {:?}", brewer.get_tea());
            } else if let Some(pour) = step.as_any().downcast_ref::<Pour>() {
                println!("Pour operation!");
                let _tea = pour.exec(brewer.get_tea());
            }
        }

        for step in self.recipe.iter() {
            step.print();
        }
        3
    }
}

impl Brewer {
    pub fn new() -> Brewer {
        let tea = Tea::new();
        Brewer { tea }
    }
    pub fn get_tea(&self) -> &Tea {
        &self.tea
    }
    fn update_brew(&mut self, tea: Tea) {
        self.tea = tea;
    }
}

// To be able to pattern match, need to define keys being mapped to from Fill
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RawTea1 {
    x: i32,
    str_val: String,
    y: bool,
}

///
/// Raw data inputs
pub trait Source {
    ///
    /// Currently this outputs Tea, in the future it will pull in all desired data, pushing it in
    /// batches to a source that the Brewers pull from.
    fn collect(&self) -> Tea;
    fn as_any(&self) -> &dyn Any;
    fn print(&self);
}

pub trait Ingredient<'a> {
    fn exec(&self, tea: &Tea) -> Tea;
    fn print(&self); 
    fn as_any(&self) -> &dyn Any;
}

pub struct Fill{
    pub source: String,
    pub name: String,
}

pub struct Transfuse;

pub struct Steep{
    pub name: String,
}

pub struct Skim{
    pub name: String,
}

pub struct Pour{
    pub name: String,
}

impl Source for Fill {
    fn collect(&self) -> Tea {
        let data = r#"{
            "x": 10000,
            "str_val": "new_values",
            "y": false
        }"#;
        let data: RawTea1 = serde_json::from_str(data).unwrap();
        Tea { data }
    }
    fn print(&self) {
        println!("Current Step: {}", self.name);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<'a> Ingredient<'a> for Steep {
    // TODO: remap existing tea, or efficiently copy over non-changed values
    fn exec(&self, tea: &Tea) -> Tea {
        let x = tea.data.x;
        let x = x - 1234567;
        let new_tea = Tea { data: RawTea1 { x, str_val: String::from(&tea.data.str_val[..]), y: false } };
        new_tea

    }
    fn print(&self) {
        println!("Current Step: {}", self.name);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<'a> Ingredient<'a> for Pour {
    fn print(&self) {
        println!("Current Step: {}", self.name);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn exec(&self, _tea: &Tea) -> Tea {
        println!("Dumped tea out! Oops");
        Tea { data: RawTea1 { x: 1, str_val: String::from("test"), y: false } }
    }
}

#[derive(Debug, PartialEq)]
/// Resulting data that is being manipulated in the brew.
pub struct Tea {
    data: RawTea1
}

impl Tea {
    /// Temporarily, new creates sample data to test constructing the recipe and adding it to the
    /// Pot. In the future, Fill will result in data that is passed on to the processes to be
    /// brewed.
    pub fn new() -> Tea {
        let data = r#"{
          "x": 1,
          "str_val": "new_values",
          "y": false
        }"#;
        let data: RawTea1 = serde_json::from_str(data).unwrap();
        Tea { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_pot() {
        let new_pot = Pot::new();
        assert_eq!(new_pot.recipe, Vec::new());
    }

    #[test]
    fn create_pot_with_recipe() {
        let mut new_pot = Pot::new();
        new_pot.add(Box::new(Fill));
        new_pot.add(Box::new(Steep));
        new_pot.add(Box::new(Pour));
        assert_eq!(new_pot.recipe, vec![Box::new(Fill), Box::new(Steep), Box::new(Pour)]);
    }

    #[test]
    fn brew_recipe() {
        let mut new_pot = Pot::new();
        new_pot.add(Box::new(Fill));
        new_pot.add(Box::new(Steep));
        new_pot.add(Box::new(Pour));
        assert_eq!(new_pot.brew(), 3);
    }
}
