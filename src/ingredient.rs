use std::any::Any;

pub use super::tea::Tea;

pub trait Ingredient<'a> {
    fn exec(&self, tea: &Box<dyn Tea>) -> Box<dyn Tea>;
    fn print(&self); 
    fn as_any(&self) -> &dyn Any;
    fn get_name(&self) -> &str;
}

pub trait Argument {
    fn as_any(&self) -> &dyn Any;
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
    pub computation: Box<Fn(&Option<Box<dyn Argument>>) -> Box<dyn Tea>>,
    pub params: Option<Box<dyn Argument>>,
}

pub struct Transfuse;

pub struct Steep {
    pub name: String,
    pub computation: Box<Fn(&Box<dyn Tea>, &Option<Box<dyn Argument>>) -> Box<dyn Tea>>, 
    pub params: Option<Box<dyn Argument>>,
}

pub struct Skim {
    pub name: String,
    pub computation: Box<Fn(&Box<dyn Tea>, &Option<Box<dyn Argument>>) -> Box<dyn Tea>>, 
    pub params: Option<Box<dyn Argument>>,
}

pub struct Pour{
    pub name: String,
    pub computation: Box<Fn(&Box<dyn Tea>, &Option<Box<dyn Argument>>) -> Box<dyn Tea>>, 
    pub params: Option<Box<dyn Argument>>,
}

impl Fill {
    pub fn get_params(&self) -> &Option<Box<dyn Argument>> {
        &self.params
    }
}

impl Steep {
    pub fn get_params(&self) -> &Option<Box<dyn Argument>> {
        &self.params
    }
}

impl Skim {
    pub fn get_params(&self) -> &Option<Box<dyn Argument>> {
        &self.params
    }
}

impl Pour {
    pub fn get_params(&self) -> &Option<Box<dyn Argument>> {
        &self.params
    }
}

impl<'a> Ingredient<'a> for Steep {
    fn exec(&self, tea: &Box<dyn Tea>) -> Box<dyn Tea> {
        (self.computation)(tea, self.get_params())
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

impl<'a> Ingredient<'a> for Skim {
    fn exec(&self, tea: &Box<dyn Tea>) -> Box<dyn Tea> {
        (self.computation)(tea, self.get_params())
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
    fn exec(&self, tea: &Box<dyn Tea>) -> Box<dyn Tea> {
        (self.computation)(tea, self.get_params())
    }
}
