// TODO: File needs reorganizing!
//
//use serde::{Deserialize, Serialize};
//use std::fmt;
//use serde_json::Result;

pub mod brewer;
pub mod tea;
pub mod ingredient;
pub mod source;

use brewer::Brewer;
use ingredient::{Ingredient, Fill};
use source::Source;

/// Data Structure that holds the recipe to brew tea (ETL data).
pub struct Pot<'a> {
    recipe:  Vec<Box<dyn Ingredient<'a>>>,
    sources: Vec<Box<dyn Source>>,
}

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

    pub fn get_sources(&self) -> &Vec<Box<dyn Source>> {
        &self.sources
    }

    pub fn get_recipe(&self) -> &Vec<Box<dyn Ingredient<'a>>> {
        &self.recipe
    }

    ///
    /// This runs the recipe to transform data.
    pub fn brew(&self, mut brewer: Brewer) {
        let init_tea = brewer.get_tea();
        println!("Initial tea: {:?}", init_tea);
        let source = &self.sources[0]; 
        source.print();
        let fill = source.as_any().downcast_ref::<Fill>().unwrap();
        let tea = fill.collect();

        brewer.make_tea(self.get_recipe(), tea);
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
