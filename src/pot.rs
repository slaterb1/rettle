use super::ingredient::{Ingredient, Fill};
use super::source::Source;
use super::brewer::Brewery;

use std::sync::{Arc, Mutex};

/// Data Structure that holds the recipe to brew tea (ETL data).
pub struct Pot<'a> {
    recipe:  Arc<Mutex<Vec<Box<dyn Ingredient<'a> + Send>>>>,
    sources: Vec<Box<dyn Source>>,
}

impl<'a> Pot<'a> {
    ///
    /// Initializes Pot with an empty recipe.
    pub fn new() -> Pot<'a> {
        Pot { recipe: Arc::new(Mutex::new(Vec::new())), sources: Vec::new() }
    }

    ///
    /// The ingredient is the instruction being added to the brew.
    pub fn add_ingredient(self, ingredient: Box<dyn Ingredient<'a> + Send>) {
        let mut recipe = self.recipe.lock().unwrap();
        recipe.push(ingredient);
    }

    pub fn add_source(&mut self, source: Box<dyn Source>) {
        &self.sources.push(source);
    }

    pub fn get_sources(&self) -> &Vec<Box<dyn Source>> {
        &self.sources
    }

    pub fn get_recipe(&self) -> Arc<Mutex<Vec<Box<dyn Ingredient<'a> + Send>>>> {
        Arc::clone(&self.recipe)
    }

    ///
    /// This runs the recipe to transform data.
    pub fn brew(&self, brewery: &Brewery) {
        let source = &self.sources[0]; 
        source.print();
        let fill = source.as_any().downcast_ref::<Fill>().unwrap();
        fill.collect(brewery, self.get_recipe());
    }
}

//impl<'a> PartialEq for Pot<'a> {
//    fn eq(&self, other: &Pot) -> bool {
//        if self.recipe.len() != other.recipe.len() {
//            return false;
//        }
//        for (i, item) in self.recipe.iter().enumerate() {
//            if **item != *other.recipe[i] {
//                return false;
//            }
//        }
//        true
//    }
//}

#[cfg(test)]
mod tests {
    use super::Pot;
    use super::super::ingredient::{Fill, Steep, Pour, Argument};
    use super::super::tea::Tea;
    use std::any::Any;

    #[derive(Debug, PartialEq, Default)]
    struct TestTea {
        x: i32,
    }

    impl Tea for TestTea {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn new(self: Box<Self>) -> Box<dyn Tea> {
            Box::new(TestTea::default())
        }
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
        assert_eq!(new_pot.get_recipe().len(), 0);
    }

    #[test]
    fn create_pot_with_source() {
        let mut new_pot = Pot::new();
        new_pot.add_source(Box::new(Fill{
            name: String::from("fake_tea"),
            source: String::from("hardcoded"),
            computation: Box::new(|_args| {
                TestTea::new(Box::new(TestTea::default()))
            }),
            params: None,
        }));
        assert_eq!(new_pot.get_sources().len(), 1);
        assert_eq!(new_pot.get_sources()[0].get_name(), "fake_tea");
    }

    #[test]
    fn create_pot_with_recipe() {
        let mut new_pot = Pot::new();
        new_pot.add_ingredient(Box::new(Steep{
            name: String::from("steep1"),
            computation: Box::new(|_tea, _args| {
                TestTea::new(Box::new(TestTea::default()))
            }),
            params: None,
        }));
        new_pot.add_ingredient(Box::new(Pour{
            name: String::from("pour1"),
            computation: Box::new(|_tea, _args| {
                TestTea::new(Box::new(TestTea::default()))
            }),
            params: None,
        }));
        assert_eq!(new_pot.get_recipe().len(), 2);
        assert_eq!(new_pot.get_recipe()[0].get_name(), "steep1");
        assert_eq!(new_pot.get_recipe()[1].get_name(), "pour1");
    }

    #[test]
    fn create_pot_with_recipe_and_optional_params() {
        let mut new_pot = Pot::new();
        new_pot.add_ingredient(Box::new(Steep{
            name: String::from("steep1"),
            computation: Box::new(|_tea, _args| {
                TestTea::new(Box::new(TestTea::default()))
            }),
            params: Some(Box::new(TestArgs::default())),
        }));
        new_pot.add_ingredient(Box::new(Pour{
            name: String::from("pour1"),
            computation: Box::new(|_tea, _args| {
                TestTea::new(Box::new(TestTea::default()))
            }),
            params: None,
        }));
        assert_eq!(new_pot.get_recipe().len(), 2);
        assert_eq!(new_pot.get_recipe()[0].get_name(), "steep1");
        assert_eq!(new_pot.get_recipe()[1].get_name(), "pour1");
    }

    #[test]
    fn create_pot_with_source_and_recipe() {
        let mut new_pot = Pot::new();
        new_pot.add_source(Box::new(Fill{
            name: String::from("fake_tea"),
            source: String::from("hardcoded"),
            computation: Box::new(|_args| {
                TestTea::new(Box::new(TestTea::default()))
            }),
            params: None,
        }));
        new_pot.add_ingredient(Box::new(Steep{
            name: String::from("steep1"),
            computation: Box::new(|_tea, _args| {
                TestTea::new(Box::new(TestTea::default()))
            }),
            params: None,
        }));
        new_pot.add_ingredient(Box::new(Pour{
            name: String::from("pour1"),
            computation: Box::new(|_tea, _args| {
                TestTea::new(Box::new(TestTea::default()))
            }),
            params: None,
        }));
        assert_eq!(new_pot.get_sources().len(), 1);
        assert_eq!(new_pot.get_recipe().len(), 2);
        assert_eq!(new_pot.get_sources()[0].get_name(), "fake_tea");
        assert_eq!(new_pot.get_recipe()[0].get_name(), "steep1");
        assert_eq!(new_pot.get_recipe()[1].get_name(), "pour1");
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

