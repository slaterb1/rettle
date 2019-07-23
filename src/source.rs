pub use crate::tea::Tea;
pub use crate::ingredient::{Fill, Ingredient};
pub use crate::brewer::Brewery;

use std::any::Any;
use std::sync::{Arc, RwLock};

///
/// Raw data inputs
pub trait Source {
    ///
    /// Currently this outputs Tea, in the future it will pull in all desired data, pushing it in
    /// batches to a source that the Brewers pull from.
    fn collect(&self, brewer: &Brewery, recipe: Arc<RwLock<Vec<Box<dyn Ingredient + Send + Sync>>>>);
    fn as_any(&self) -> &dyn Any;
    fn print(&self);
    fn get_name(&self) -> &str;
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


