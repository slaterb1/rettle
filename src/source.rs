use std::any::Any;

pub use super::tea::Tea;
pub use super::ingredient::{Fill, Ingredient};
pub use super::brewer::Brewery;

///
/// Raw data inputs
pub trait Source {
    ///
    /// Currently this outputs Tea, in the future it will pull in all desired data, pushing it in
    /// batches to a source that the Brewers pull from.
    fn collect(&self, brewer: &Brewery, recipe: &Vec<Box<dyn Ingredient>>);
    fn as_any(&self) -> &dyn Any;
    fn print(&self);
    fn get_name(&self) -> &str;
}

impl Source for Fill {
    fn collect(&self, brewery: &Brewery, recipe: &Vec<Box<dyn Ingredient>>) {
        (self.computation)(self.get_params(), brewery, recipe)
    }
    fn get_name(&self) -> &str {
        &self.name[..]
    }
    fn print(&self) {
        println!("Current Source: {}", self.get_name());
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


