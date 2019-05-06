use serde_json::Value;

#[derive(Debug, PartialEq)]
/// Data Structure that holds the recipe to brew tea (ETL data).
pub struct Pot<'a> {
    recipe: Vec<Ingredient<'a>>,
}

impl<'a> Pot<'a> {
    ///
    /// Initializes Pot with an empty recipe.
    pub fn new() -> Pot<'a> {
        Pot { recipe: Vec::new() }
    }

    ///
    /// The ingredient is the instruction being added to the brew.
    pub fn add(&mut self, ingredient: Ingredient<'a>) {
        &self.recipe.push(ingredient);
    }
}

#[derive(Debug, PartialEq)]
/// Data Structure defining types of Ingredients that can be added to the brew.
pub enum Ingredient<'a> {
    Fill,
    Transfuse(Vec<Tea>),
    Steep(&'a Tea),
    Skim(&'a Tea),
    Pour(&'a Tea),
}

#[derive(Debug, PartialEq)]
/// Resulting data that is being manipulated in the brew.
pub struct Tea {
    data: serde_json::Value,
}

impl Tea {
    /// Temporarily, new creates sample data to test constructing the recipe and adding it to the
    /// Pot. In the future, Fill will result in data that is passed on to the processes to be
    /// brewed.
    pub fn new() -> Tea {
        let data = r#"{
            "x": 1,
            "str_val": "test",
            "y": false
        }"#;
        let data: Value = serde_json::from_str(data).unwrap();
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
        let tea = Tea::new();
        new_pot.add(Ingredient::Fill);
        new_pot.add(Ingredient::Steep(&tea));
        new_pot.add(Ingredient::Pour(&tea));
        assert_eq!(new_pot.recipe, vec![Ingredient::Fill, Ingredient::Steep(&tea), Ingredient::Pour(&tea)]);
    }
}
