pub use super::tea::Tea;
pub use super::ingredient::{Ingredient, Steep, Pour};

/// Worker that runs the recipe and brew tea.
pub struct Brewer {
    tea: Box<dyn Tea>,
}

impl Brewer {
    pub fn new(tea: Box<dyn Tea>) -> Brewer {
        Brewer { tea }
    }
    pub fn get_tea(&self) -> &Box<dyn Tea> {
        &self.tea
    }
    fn update_brew(&mut self, tea: Box<dyn Tea>) {
        self.tea = tea;
    }
    ///
    /// This function iterates over the brewer's steps to produce the final tea.
    pub fn make_tea(&mut self, recipe: &Vec<Box<Ingredient>>) {
        // Save initial state of tea in brewer
        for step in recipe.iter() {
            step.print();
            if let Some(steep) = step.as_any().downcast_ref::<Steep>() {
                println!("Steep operation!");
                let tea = steep.exec(self.get_tea());
                self.update_brew(tea);
            } else if let Some(pour) = step.as_any().downcast_ref::<Pour>() {
                println!("Pour operation!");
                let _tea = pour.exec(self.get_tea());
            }
        }
    }
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


