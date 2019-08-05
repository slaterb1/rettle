use crate::brewery::Brewery;
use crate::tea::Tea;

use std::any::Any;
use std::sync::{Arc, RwLock};

///
/// Trait given to Box elements added to Pot for pulling, processing, or sending data.
pub trait Ingredient {
    ///
    /// Run computation on batch of Tea.
    ///
    /// # Arguements
    ///
    /// * `tea_batch` - current tea batch to be processed
    fn exec(&self, tea_batch: Vec<Box<dyn Tea + Send>>) -> Vec<Box<dyn Tea + Send>>;

    ///
    /// Print out current step information.
    fn print(&self); 

    ///
    /// Used to convert Box<dyn Ingredient> to Any to unwrap Ingredient. 
    fn as_any(&self) -> &dyn Any;

    ///
    /// Returns name given to Ingredient.
    fn get_name(&self) -> &str;
}

///
/// Trait given to Box elements that add params to Ingredients.
pub trait Argument {
    fn as_any(&self) -> &dyn Any;
}

///
/// Ingredient used to import or create Tea used in the Pot.
pub struct Fill {
    pub source: String,
    pub name: String,
    pub computation: Box<fn(&Option<Box<dyn Argument + Send>>, &Brewery, Arc<RwLock<Vec<Box<dyn Ingredient + Send + Sync>>>>)>,
    pub params: Option<Box<dyn Argument + Send>>,
}

///
/// Ingredient used to combine Tea pulled from multiple Fill sources. *Not currently implemented*
pub struct Transfuse;

///
/// Ingredient used to transform Tea in the Pot.
pub struct Steep {
    pub name: String,
    pub computation: Box<fn(Vec<Box<dyn Tea + Send>>, &Option<Box<dyn Argument + Send>>) -> Vec<Box<dyn Tea + Send>>>, 
    pub params: Option<Box<dyn Argument + Send>>,
}

///
/// Ingredient used to remove fields on Tea in the Pot. *Not currently implemented*
pub struct Skim {
    pub name: String,
    pub computation: Box<fn(Vec<Box<dyn Tea + Send>>, &Option<Box<dyn Argument + Send>>) -> Vec<Box<dyn Tea + Send>>>, 
    pub params: Option<Box<dyn Argument + Send>>,
}

///
/// Ingredient used to send Tea to somewhere else.
pub struct Pour{
    pub name: String,
    pub computation: Box<fn(Vec<Box<dyn Tea + Send>>, &Option<Box<dyn Argument + Send>>) -> Vec<Box<dyn Tea + Send>>>, 
    pub params: Option<Box<dyn Argument + Send>>,
}

impl Fill {
    ///
    /// Return params, if any, initialized to this step.
    pub fn get_params(&self) -> &Option<Box<dyn Argument + Send>> {
        &self.params
    }
}

impl Steep {
    ///
    /// Return params, if any, initialized to this step.
    pub fn get_params(&self) -> &Option<Box<dyn Argument + Send>> {
        &self.params
    }
}

impl Skim {
    ///
    /// Return params, if any, initialized to this step.
    pub fn get_params(&self) -> &Option<Box<dyn Argument + Send>> {
        &self.params
    }
}

impl Pour {
    ///
    /// Return params, if any, initialized to this step.
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
    fn exec(&self, tea_batch: Vec<Box<dyn Tea + Send>>) -> Vec<Box<dyn Tea + Send>> {
        (self.computation)(tea_batch, self.get_params())
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
    fn exec(&self, tea_batch: Vec<Box<dyn Tea + Send>>) -> Vec<Box<dyn Tea + Send>> {
        (self.computation)(tea_batch, self.get_params())
    }
}

// TODO: Implement Ingredient for Fill (add step plus logic to `brewery::make_tea` function)

// TODO: Implement Ingredient for Skim (add step plus logic to `brewery::make_tea` function)

// TODO: Implement Ingredient for Transfuse (add step plus logic to `brewery::make_tea` function)

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
        assert_eq!(fill.get_source(), "text");
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
        assert_eq!(fill.get_source(), "text");
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
        let orig_tea = vec![Box::new(TestTea::default()) as Box<dyn Tea + Send>];
        let orig_tea_copy = vec![Box::new(TestTea::default()) as Box<dyn Tea + Send>];
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
        let orig_tea = vec![Box::new(TestTea::default()) as Box<dyn Tea + Send>];
        let orig_tea_copy = vec![Box::new(TestTea::default()) as Box<dyn Tea + Send>];
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
        let orig_tea = vec![Box::new(TestTea::default()) as Box<dyn Tea + Send>];
        let new_tea = pour.exec(orig_tea);
        let orig_tea_copy = vec![Box::new(TestTea::default()) as Box<dyn Tea + Send>];
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
        let orig_tea = vec![Box::new(TestTea::default()) as Box<dyn Tea + Send>];
        let orig_tea_copy = vec![Box::new(TestTea::default()) as Box<dyn Tea + Send>];
        let new_tea = pour.exec(orig_tea);
        let orig_tea = orig_tea_copy[0].as_any().downcast_ref::<TestTea>().unwrap();
        let new_tea = new_tea[0].as_any().downcast_ref::<TestTea>().unwrap();
        assert_eq!(pour.get_name(), "test_pour");
        assert_eq!(new_tea.x, orig_tea.x);
    }
}
