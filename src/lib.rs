// TODO: File needs reorganizing!
//
//use serde::{Deserialize, Serialize};
//use std::fmt;
//use serde_json::Result;

pub mod brewer;
pub mod tea;
pub mod ingredient;
pub mod source;
pub mod pot;

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
