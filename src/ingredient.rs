use std::any::Any;

pub use super::tea::{Tea, RawTea1};

pub trait Ingredient<'a> {
    fn exec(&self, tea: &Tea) -> Tea;
    fn print(&self); 
    fn as_any(&self) -> &dyn Any;
    fn get_name(&self) -> &str;
}

//impl<'a> PartialEq for &'a Ingredient<'a> {
//    fn eq(&self, other: &Self) -> bool {
//        let tea = Tea::new();
//        if self.exec(&tea) != other.exec(&tea) {
//            return false;
//        }
//        true
//    }
//}

pub struct Fill{
    pub source: String,
    pub name: String,
}

pub struct Transfuse;

pub struct Steep {
    pub name: String,
    pub computation: Box<Fn(&Tea) -> Tea> 
}

pub struct Skim{
    pub name: String,
}

pub struct Pour{
    pub name: String,
    pub computation: Box<Fn(&Tea) -> Tea> 
}

impl<'a> Ingredient<'a> for Steep {
    // TODO: remap existing tea, or efficiently copy over non-changed values
    fn exec(&self, tea: &Tea) -> Tea {
        (self.computation)(tea)
    }
    fn get_name(&self) -> &str {
        &self.name[..]
    }
    fn print(&self) {
        println!("Current Step: {}", self.get_name());
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<'a> Ingredient<'a> for Pour {
    fn get_name(&self) -> &str {
        &self.name[..]
    }
    fn print(&self) {
        println!("Current Step: {}", self.get_name());
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn exec(&self, tea: &Tea) -> Tea {
        (self.computation)(tea)
    }
}
