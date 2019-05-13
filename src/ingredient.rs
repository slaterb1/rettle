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

pub struct Steep<T>
    where T: Fn(&Tea) -> Tea
{
    pub name: String,
    pub calculation: T
}

pub struct Skim{
    pub name: String,
}

pub struct Pour{
    pub name: String,
}

impl<'a, T> Ingredient<'a> for Steep<T> 
    where T: Fn(&Tea) -> Tea
{
    // TODO: remap existing tea, or efficiently copy over non-changed values
    fn exec(&self, tea: &Tea) -> Tea {
        //let x = tea.data.x;
        //let x = x - 1234567;
        //let new_tea = Tea { data: RawTea1 { x, str_val: String::from(&tea.data.str_val[..]), y: false } };
        //new_tea
        (self.calculation)(tea)

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
    fn exec(&self, _tea: &Tea) -> Tea {
        println!("Dumped tea out! Oops");
        Tea { data: RawTea1 { x: 1, str_val: String::from("test"), y: false } }
    }
}
