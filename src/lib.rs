use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Debug, PartialEq)]
/// Data Structure that holds the recipe to brew tea (ETL data).
pub struct Pot<'a> {
    recipe: Vec<Ingredient<'a>>,
    tea: Tea<'a>,
}

impl<'a> Pot<'a> {
    ///
    /// Initializes Pot with an empty recipe.
    pub fn new() -> Pot<'a> {
        Pot { recipe: Vec::new(), tea: Tea::new() }
    }

    ///
    /// The ingredient is the instruction being added to the brew.
    pub fn add(&mut self, ingredient: Ingredient<'a>) {
        &self.recipe.push(ingredient);
    }

    fn get_tea(&self) -> &'a Tea {
        &self.tea
    }

    ///
    /// This runs the recipe to transform data.
    pub fn brew(&self) -> u32 {
        for i in &self.recipe {
            match i {
                Ingredient::Fill => println!("Fill operation!"),
                Ingredient::Steep => println!("Steep operation, with tea: {:?}!", &self.get_tea()),
                Ingredient::Pour => println!("Pour operation, with tea: {:?}!", &self.get_tea()),
                _ => println!("Some other operation")
            }
        }
        3
    }
}

// To be able to pattern match, need to define keys being mapped to from Fill
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RawTea1<'a> {
    x: i32,
    str_val: &'a str,
    y: bool,
}

#[derive(Debug, PartialEq)]
/// Data Structure defining types of Ingredients that can be added to the brew.
pub enum Ingredient<'a> {
    Fill,
    Transfuse(Vec<Tea<'a>>),
    Steep,
    Skim,
    Pour,
}

pub struct Fill {
}

pub struct Transfuse {
}

pub struct Steep {
}

pub struct Skim {
}

pub struct Pour {
}

impl<'a> Fill {
    fn fill() -> Tea<'a> {
        let data = r#"{
            "x": 1,
            "str_val": "test",
            "y": false
        }"#;
        let data: RawTea1 = serde_json::from_str(data).unwrap();
        Tea { data }
    }
}

impl Steep {
    fn steep(tea: Tea) -> Tea {
        match tea.data.x {
            1 => println!("{:?}", 1),
            _ => println!("Does not have key \"x\"")
        }
        tea
    }
}

#[derive(Debug, PartialEq)]
/// Resulting data that is being manipulated in the brew.
pub struct Tea<'a> {
    data: RawTea1<'a>
}

impl<'a> Tea<'a> {
    /// Temporarily, new creates sample data to test constructing the recipe and adding it to the
    /// Pot. In the future, Fill will result in data that is passed on to the processes to be
    /// brewed.
    pub fn new() -> Tea<'a> {
        let data = r#"{
            "x": 1,
            "str_val": "test",
            "y": false
        }"#;
        let data: RawTea1 = serde_json::from_str(data).unwrap();
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

    #[test]
    fn brew_recipe() {
        let mut new_pot = Pot::new();
        let tea = Tea::new();
        new_pot.add(Ingredient::Fill);
        new_pot.add(Ingredient::Steep(&tea));
        new_pot.add(Ingredient::Pour(&tea));
        assert_eq!(new_pot.brew(), 3);
    }
}
