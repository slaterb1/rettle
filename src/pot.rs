use crate::ingredient::{Ingredient, Fill};
use crate::source::Source;
use crate::brewery::Brewery;

use std::sync::{Arc, RwLock};

/// Data Structure that holds the recipe to brew tea (ETL data).
pub struct Pot<T: Send> {
    recipe:  Arc<RwLock<Vec<Box<dyn Ingredient<T> + Send + Sync>>>>,
    sources: Vec<Box<dyn Source<T>>>,
}

impl<T: Send + 'static> Pot<T> {
    ///
    /// Initializes Pot with an empty recipe and empty sources.
    pub fn new() -> Pot<T> {
        Pot { recipe: Arc::new(RwLock::new(Vec::new())), sources: Vec::new() }
    }

    ///
    /// Adds Ingredient to recipe held by the Pot.
    ///
    /// # Arguments
    ///
    /// * `ingredient` - the ingredient to add to the recipe
    pub fn add_ingredient(&self, ingredient: Box<dyn Ingredient<T> + Send + Sync>) {
        let mut recipe = self.recipe.write().unwrap();
        recipe.push(ingredient);
    }

    ///
    /// Adds Source to sources held by the Pot.
    ///
    /// # Arguments
    ///
    /// * `source` - the source to add to the sources Array
    pub fn add_source(&mut self, source: Box<dyn Source<T>>) {
        &self.sources.push(source);
    }

    /// 
    /// Returns the sources held by the Pot.
    pub fn get_sources(&self) -> &Vec<Box<dyn Source<T>>> {
        &self.sources
    }

    /// 
    /// Returns the recipe held by the Pot.
    pub fn get_recipe(&self) -> Arc<RwLock<Vec<Box<dyn Ingredient<T> + Send + Sync>>>> {
        Arc::clone(&self.recipe)
    }

    ///
    /// Iterates over sources to pull in data and send jobs to the Brewery for processing.
    ///
    /// # Arguments
    ///
    /// * `brewery` - Brewery struct holding the receiver and Brewer Array to process Tea
    pub fn brew(&self, brewery: &Brewery) {
        println!("Brewing Tea...");
        for source in self.get_sources() {
            source.print();
            let fill = source.as_any().downcast_ref::<Fill<T>>().unwrap();
            fill.collect(brewery, self.get_recipe());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Pot;
    use super::super::ingredient::{Fill, Steep, Pour, Argument};
    use std::any::Any;

    #[derive(Debug, PartialEq, Default)]
    struct TestTea {
        x: i32,
    }

    #[derive(Default)]
    struct TestArgs {
        pub val: i32
    }

    impl Argument for TestArgs {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn create_empty_pot() {
        let new_pot = Pot::new();
        assert_eq!(new_pot.get_recipe().read().unwrap().len(), 0);
    }

    #[test]
    fn create_pot_with_source() {
        let mut new_pot = Pot::new();
        new_pot.add_source(Box::new(Fill{
            name: String::from("fake_tea"),
            source: String::from("hardcoded"),
            computation: Box::new(|_args, _brewery, _recipe| {
                TestTea::default();
            }),
            params: None,
        }));
        assert_eq!(new_pot.get_sources().len(), 1);
        assert_eq!(new_pot.get_sources()[0].get_name(), "fake_tea");
        assert_eq!(new_pot.get_sources()[0].get_source(), "hardcoded");
    }

    #[test]
    fn create_pot_with_recipe() {
        let new_pot = Pot::new();
        new_pot.add_ingredient(Box::new(Steep{
            name: String::from("steep1"),
            computation: Box::new(|_tea, _args| {
                vec![TestTea::default()]
            }),
            params: None,
        }));
        new_pot.add_ingredient(Box::new(Pour{
            name: String::from("pour1"),
            computation: Box::new(|_tea, _args| {
                vec![TestTea::default()]
            }),
            params: None,
        }));
        assert_eq!(new_pot.get_recipe().read().unwrap().len(), 2);
        assert_eq!(new_pot.get_recipe().read().unwrap()[0].get_name(), "steep1");
        assert_eq!(new_pot.get_recipe().read().unwrap()[1].get_name(), "pour1");
    }

    #[test]
    fn create_pot_with_recipe_and_optional_params() {
        let new_pot = Pot::new();
        new_pot.add_ingredient(Box::new(Steep{
            name: String::from("steep1"),
            computation: Box::new(|_tea, _args| {
                vec![TestTea::default()]
            }),
            params: Some(Box::new(TestArgs::default())),
        }));
        new_pot.add_ingredient(Box::new(Pour{
            name: String::from("pour1"),
            computation: Box::new(|_tea, _args| {
                vec![Box::new(TestTea::default()) as Box<dyn Tea + Send>]
            }),
            params: None,
        }));
        assert_eq!(new_pot.get_recipe().read().unwrap().len(), 2);
        assert_eq!(new_pot.get_recipe().read().unwrap()[0].get_name(), "steep1");
        assert_eq!(new_pot.get_recipe().read().unwrap()[1].get_name(), "pour1");
    }

    #[test]
    fn create_pot_with_source_and_recipe() {
        let mut new_pot = Pot::new();
        new_pot.add_source(Box::new(Fill{
            name: String::from("fake_tea"),
            source: String::from("hardcoded"),
            computation: Box::new(|_args, _brewery, _recipe| {
                TestTea::default();
            }),
            params: None,
        }));
        new_pot.add_ingredient(Box::new(Steep{
            name: String::from("steep1"),
            computation: Box::new(|_tea, _args| {
                vec![TestTea::default()]
            }),
            params: None,
        }));
        new_pot.add_ingredient(Box::new(Pour{
            name: String::from("pour1"),
            computation: Box::new(|_tea, _args| {
                vec![TestTea::default()]
            }),
            params: None,
        }));
        assert_eq!(new_pot.get_sources().len(), 1);
        assert_eq!(new_pot.get_recipe().read().unwrap().len(), 2);
        assert_eq!(new_pot.get_sources()[0].get_name(), "fake_tea");
        assert_eq!(new_pot.get_sources()[0].get_source(), "hardcoded");
        assert_eq!(new_pot.get_recipe().read().unwrap()[0].get_name(), "steep1");
        assert_eq!(new_pot.get_recipe().read().unwrap()[1].get_name(), "pour1");
    }

    //TODO: Readd test after returning Result
    //#[test]
    //fn brew_recipe() {
    //    let mut new_pot = Pot::new();
    //    new_pot.add(Box::new(Fill));
    //    new_pot.add(Box::new(Steep));
    //    new_pot.add(Box::new(Pour));
    //    assert_eq!(new_pot.brew(), 3);
    //}
}

