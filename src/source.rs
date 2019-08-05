use crate::ingredient::{Fill, Ingredient};
use crate::brewer::Brewery;

use std::any::Any;
use std::sync::{Arc, RwLock};

///
/// Trait given to Box elements added to Pot for pulling in raw data.
pub trait Source {
    ///
    /// Runs the Fill computation to collect Tea in batches and send to the Brewery for processing.
    ///
    /// # Arguments
    ///
    /// * `brewery` - Brewery that sends job to process Tea
    /// * `recipe` - clone of recipe to pass to Brewery
    fn collect(&self, brewery: &Brewery, recipe: Arc<RwLock<Vec<Box<dyn Ingredient + Send + Sync>>>>);

    ///
    /// Used to convert Box<dyn Ingredient> to Any to unwrap Ingredient. 
    fn as_any(&self) -> &dyn Any;

    ///
    /// Print out current step information.
    fn print(&self);

    ///
    /// Returns name given to Ingredient.
    fn get_name(&self) -> &str;

    ///
    /// Returns source given to Ingredient.
    fn get_source(&self) -> &str;
}

impl Source for Fill {
    fn collect(&self, brewery: &Brewery, recipe: Arc<RwLock<Vec<Box<dyn Ingredient + Send + Sync>>>>) {
        (self.computation)(self.get_params(), brewery, recipe)
    }
    fn get_name(&self) -> &str {
        &self.name[..]
    }
    fn get_source(&self) -> &str {
        &self.source[..]
    }
    fn print(&self) {
        println!("Current Source: {}", self.get_name());
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


