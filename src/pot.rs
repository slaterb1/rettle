use super::ingredient::{Ingredient, Fill};
use super::source::Source;
use super::brewer::Brewer;

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


