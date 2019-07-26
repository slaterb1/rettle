use crate::brewer::Brewery;
use crate::tea::Tea;

use std::any::Any;
use std::sync::{Arc, RwLock};

pub trait Ingredient {
    fn exec(&self, tea: Vec<Box<dyn Tea + Send>>) -> Vec<Box<dyn Tea + Send>>;
    fn print(&self); 
    fn as_any(&self) -> &dyn Any;
    fn get_name(&self) -> &str;
}

pub trait Argument {
    fn as_any(&self) -> &dyn Any;
}

pub struct Fill{
    pub source: String,
    pub name: String,
    pub computation: Box<Fn(&Option<Box<dyn Argument + Send>>, &Brewery, Arc<RwLock<Vec<Box<dyn Ingredient + Send + Sync>>>>)>,
    pub params: Option<Box<dyn Argument + Send>>,
}

pub struct Transfuse;

pub struct Steep {
    pub name: String,
    pub computation: Box<Fn(Vec<Box<dyn Tea + Send>>, &Option<Box<dyn Argument + Send>>) -> Vec<Box<dyn Tea + Send>>>, 
    pub params: Option<Box<dyn Argument + Send>>,
}

pub struct Skim {
    pub name: String,
    pub computation: Box<Fn(Vec<Box<dyn Tea + Send>>, &Option<Box<dyn Argument + Send>>) -> Vec<Box<dyn Tea + Send>>>, 
    pub params: Option<Box<dyn Argument + Send>>,
}

pub struct Pour{
    pub name: String,
    pub computation: Box<Fn(Vec<Box<dyn Tea + Send>>, &Option<Box<dyn Argument + Send>>) -> Vec<Box<dyn Tea + Send>>>, 
    pub params: Option<Box<dyn Argument + Send>>,
}

impl Fill {
    pub fn get_params(&self) -> &Option<Box<dyn Argument + Send>> {
        &self.params
    }
}

impl Steep {
    pub fn get_params(&self) -> &Option<Box<dyn Argument + Send>> {
        &self.params
    }
}

impl Skim {
    pub fn get_params(&self) -> &Option<Box<dyn Argument + Send>> {
        &self.params
    }
}

impl Pour {
    pub fn get_params(&self) -> &Option<Box<dyn Argument + Send>> {
        &self.params
    }
}

unsafe impl Send for Steep {}
unsafe impl Sync for Steep {}
unsafe impl Send for Skim {}
unsafe impl Sync for Skim {}
unsafe impl Send for Pour {}
unsafe impl Sync for Pour {}

impl Ingredient for Steep {
    fn exec(&self, tea: Vec<Box<dyn Tea + Send>>) -> Vec<Box<dyn Tea + Send>> {
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

impl Ingredient for Skim {
    fn exec(&self, tea: Vec<Box<dyn Tea + Send>>) -> Vec<Box<dyn Tea + Send>> {
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

impl Ingredient for Pour {
    fn get_name(&self) -> &str {
        &self.name[..]
    }
    fn print(&self) {
        println!("Current Step: {}", self.get_name());
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn exec(&self, tea: Vec<Box<dyn Tea + Send>>) -> Vec<Box<dyn Tea + Send>> {
        (self.computation)(tea, self.get_params())
    }
}

#[cfg(test)]
mod tests {
    use super::super::ingredient::{Fill, Steep, Pour, Argument, Ingredient};
    use super::super::tea::Tea;
    use super::super::source::Source;
    use std::any::Any;

    #[derive(Debug, PartialEq, Default, Clone)]
    struct TestTea {
        x: i32,
    }

    impl Tea for TestTea {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn new(self: Box<Self>) -> Box<dyn Tea + Send> {
            Box::new(TestTea::default())
        }
    }

    #[derive(Default)]
    struct TestArgs {
        pub val: i32
    }

    impl Argument for TestArgs {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn create_fill_no_params() {
        let fill = Fill {
            name: String::from("test_fill"),
            source: String::from("text"),
            computation: Box::new(|_args, _brewery, _recipe| {}),
            params: None,
        };
        assert_eq!(fill.get_name(), "test_fill");
    }

    #[test]
    fn create_fill_with_params() {
        let fill = Fill {
            name: String::from("test_fill"),
            source: String::from("text"),
            computation: Box::new(|_args, _brewery, _recipe| {}),
            params: Some(Box::new(TestArgs { val: 5 })),
        };
        assert_eq!(fill.get_name(), "test_fill");
    }

    #[test]
    fn create_steep_no_params() {
        let steep = Steep {
            name: String::from("test_steep"),
            computation: Box::new(|tea, _args| {
                tea.into_iter()
                   .map(|tea| {
                       let tea = tea.as_any().downcast_ref::<TestTea>().unwrap();
                       let mut new_tea = tea.clone();
                       new_tea.x = tea.x + 5;
                       Box::new(new_tea) as Box<dyn Tea + Send>
                   })
                   .collect()
            }),
            params: None,
        };
        let orig_tea = vec![TestTea::new(Box::new(TestTea::default()))];
        let orig_tea_copy = vec![TestTea::new(Box::new(TestTea::default()))];
        let new_tea = steep.exec(orig_tea);
        let orig_tea = orig_tea_copy[0].as_any().downcast_ref::<TestTea>().unwrap();
        let new_tea = new_tea[0].as_any().downcast_ref::<TestTea>().unwrap();
        assert_eq!(steep.get_name(), "test_steep");
        assert_eq!(new_tea.x, orig_tea.x + 5);
    }

    #[test]
    fn create_steep_with_params() {
        let steep = Steep {
            name: String::from("test_steep"),
            computation: Box::new(|tea, args| {
                tea.into_iter()
                   .map(|tea| {
                       let tea = tea.as_any().downcast_ref::<TestTea>().unwrap();
                       let mut new_tea = tea.clone();
                       match args {
                           None => println!("Nothing"),
                           Some(box_args) => {
                               let box_args = box_args.as_any().downcast_ref::<TestArgs>().unwrap();
                               new_tea.x = tea.x + box_args.val;
                           }
                       }
                       Box::new(new_tea) as Box<dyn Tea + Send>
                   })
                   .collect()
            }),
            params: Some(Box::new(TestArgs { val: 10 })),
        };
        let orig_tea = vec![TestTea::new(Box::new(TestTea::default()))];
        let orig_tea_copy = vec![TestTea::new(Box::new(TestTea::default()))];
        let new_tea = steep.exec(orig_tea);
        let orig_tea = orig_tea_copy[0].as_any().downcast_ref::<TestTea>().unwrap();
        let new_tea = new_tea[0].as_any().downcast_ref::<TestTea>().unwrap();
        assert_eq!(steep.get_name(), "test_steep");
        assert_eq!(new_tea.x, orig_tea.x + 10);
    }

    #[test]
    fn create_pour_no_params() {
        let pour = Pour {
            name: String::from("test_pour"),
            computation: Box::new(|tea, _args| {
                tea.into_iter()
                   .map(|tea| {
                       let tea = tea.as_any().downcast_ref::<TestTea>().unwrap();
                       let new_tea = tea.clone();
                       Box::new(new_tea) as Box<dyn Tea + Send>
                   })
                   .collect()
            }),
            params: None,
        };
        let orig_tea = vec![TestTea::new(Box::new(TestTea::default()))];
        let new_tea = pour.exec(orig_tea);
        let orig_tea_copy = vec![TestTea::new(Box::new(TestTea::default()))];
        let orig_tea = orig_tea_copy[0].as_any().downcast_ref::<TestTea>().unwrap();
        let new_tea = new_tea[0].as_any().downcast_ref::<TestTea>().unwrap();
        assert_eq!(pour.get_name(), "test_pour");
        assert_eq!(new_tea.x, orig_tea.x);
    }

    #[test]
    fn create_pour_with_params() {
        let pour = Pour {
            name: String::from("test_pour"),
            computation: Box::new(|tea, args| {
                tea.into_iter()
                   .map(|tea| {
                       let tea = tea.as_any().downcast_ref::<TestTea>().unwrap();
                       let new_tea = tea.clone();
                       match args {
                           None => println!("Nothing"),
                           Some(_box_args) => {
                               let _box_args = _box_args.as_any().downcast_ref::<TestArgs>().unwrap();
                           }
                       }
                       Box::new(new_tea) as Box<dyn Tea + Send>
                   })
                   .collect()
            }),
            params: Some(Box::new(TestArgs { val: 10 })),
        };
        let orig_tea = vec![TestTea::new(Box::new(TestTea::default()))];
        let orig_tea_copy = vec![TestTea::new(Box::new(TestTea::default()))];
        let new_tea = pour.exec(orig_tea);
        let orig_tea = orig_tea_copy[0].as_any().downcast_ref::<TestTea>().unwrap();
        let new_tea = new_tea[0].as_any().downcast_ref::<TestTea>().unwrap();
        assert_eq!(pour.get_name(), "test_pour");
        assert_eq!(new_tea.x, orig_tea.x);
    }
}
