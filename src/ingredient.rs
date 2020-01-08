use crate::brewery::Brewery;

use std::any::Any;
use std::sync::{Arc, RwLock};

///
/// Trait given to Box elements added to Pot for pulling, processing, or sending data.
pub trait Ingredient<T: Send> {
    ///
    /// Run computation on batch of Tea.
    ///
    /// # Arguements
    ///
    /// * `tea_batch` - current tea batch to be processed
    fn exec(&self, tea_batch: Vec<T>) -> Vec<T>;

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
pub struct Fill<T: Send> {
    pub source: String,
    pub name: String,
    pub computation: Box<fn(&Option<Box<dyn Argument + Send>>, &Brewery, Arc<RwLock<Vec<Box<dyn Ingredient<T> + Send + Sync>>>>)>,
    pub params: Option<Box<dyn Argument + Send>>,
}

///
/// Ingredient used to combine Tea pulled from multiple Fill sources. *Not currently implemented*
pub struct Transfuse;

///
/// Ingredient used to transform Tea in the Pot.
pub struct Steep<T: Send> {
    pub name: String,
    pub computation: Box<fn(Vec<T>, &Option<Box<dyn Argument + Send>>) -> Vec<T>>, 
    pub params: Option<Box<dyn Argument + Send>>,
}

///
/// Ingredient used to remove fields on Tea in the Pot. *Not currently implemented*
pub struct Skim<T: Send> {
    pub name: String,
    pub computation: Box<fn(Vec<T>, &Option<Box<dyn Argument + Send>>) -> Vec<T>>, 
    pub params: Option<Box<dyn Argument + Send>>,
}

///
/// Ingredient used to send Tea to somewhere else.
pub struct Pour<T: Send> {
    pub name: String,
    pub computation: Box<fn(Vec<T>, &Option<Box<dyn Argument + Send>>) -> Vec<T>>, 
    pub params: Option<Box<dyn Argument + Send>>,
}

impl<T: Send> Fill<T> {
    ///
    /// Return params, if any, initialized to this step.
    pub fn get_params(&self) -> &Option<Box<dyn Argument + Send>> {
        &self.params
    }
}

impl<T: Send> Steep<T> {
    ///
    /// Return params, if any, initialized to this step.
    pub fn get_params(&self) -> &Option<Box<dyn Argument + Send>> {
        &self.params
    }
}

impl<T: Send> Skim<T> {
    ///
    /// Return params, if any, initialized to this step.
    pub fn get_params(&self) -> &Option<Box<dyn Argument + Send>> {
        &self.params
    }
}

impl<T: Send> Pour<T> {
    ///
    /// Return params, if any, initialized to this step.
    pub fn get_params(&self) -> &Option<Box<dyn Argument + Send>> {
        &self.params
    }
}

unsafe impl<T: Send> Send for Steep<T> {}
unsafe impl<T: Send>  Sync for Steep<T> {}
unsafe impl<T: Send>  Send for Skim<T>{}
unsafe impl<T: Send>  Sync for Skim<T> {}
unsafe impl<T: Send>  Send for Pour<T> {}
unsafe impl<T: Send>  Sync for Pour<T> {}

impl<T: Send + 'static> Ingredient<T> for Steep<T> {
    fn exec(&self, tea_batch: Vec<T>) -> Vec<T> {
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

impl<T: Send + 'static>  Ingredient<T> for Pour<T> {
    fn get_name(&self) -> &str {
        &self.name[..]
    }
    fn print(&self) {
        println!("Current Step: {}", self.get_name());
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn exec(&self, tea_batch: Vec<T>) -> Vec<T> {
        (self.computation)(tea_batch, self.get_params())
    }
}

// TODO: Implement Ingredient for Fill (add step plus logic to `brewery::make_tea` function)
// Need to consider if this still makes sense as an Ingredient in the recipe vs just a source...

impl<T: Send + 'static>  Ingredient<T> for Skim<T>  {
    fn get_name(&self) -> &str {
        &self.name[..]
    }
    fn print(&self) {
        println!("Current Step: {}", self.get_name());
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn exec(&self, tea_batch: Vec<T>) -> Vec<T> {
        (self.computation)(tea_batch, self.get_params())
    }
}

// TODO: Implement Ingredient for Transfuse (add step plus logic to `brewery::make_tea` function)

#[cfg(test)]
mod tests {
    use super::super::ingredient::{Fill, Steep, Skim, Pour, Argument, Ingredient};
    use super::super::source::Source;
    use std::any::Any;

    #[derive(Debug, PartialEq, Default, Clone)]
    struct TestTea {
        x: Option<i32>,
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

    #[derive(Default)]
    struct TestSkimArgs {
        pub field: &'static str
    }

    impl Argument for TestSkimArgs {
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
            computation: Box::new(|tea: Vec<TestTea>, _args| {
                tea.into_iter()
                   .map(|mut tea| {
                       let new_val = match new_tea.x {
                           Some(x) => Some(x + 5),
                           None => None
                       };
                       new_tea.x = new_val;
                       new_tea
                   })
                   .collect()
            }),
            params: None,
        };
        let orig_tea = vec![TestTea { x: Some(0) }];
        let orig_tea_copy = orig_tea.clone();
        let new_tea = steep.exec(orig_tea);
        assert_eq!(steep.get_name(), "test_steep");
        assert_eq!(new_tea.x.unwrap(), orig_tea.x.unwrap() + 5);
    }

    #[test]
    fn create_steep_with_params() {
        let steep = Steep {
            name: String::from("test_steep"),
            computation: Box::new(|tea: Vec<TextArgs>, args| {
                tea.into_iter()
                   .map(|mut tea| {
                       match args {
                           None => println!("Nothing"),
                           Some(box_args) => {
                               let box_args = box_args.as_any().downcast_ref::<TestArgs>().unwrap();
                               let new_val: Option<i32> = match new_tea.x {
                                   Some(x) => Some(x + box_args.val),
                                   None => None
                               };
                               new_tea.x = new_val;
                           }
                       }
                       new_tea
                   })
                   .collect()
            }),
            params: Some(Box::new(TestArgs { val: 10 })),
        };
        let orig_tea = vec![TestTea { x: Some(0) }];
        let orig_tea_copy = orig_tea.clone();
        let new_tea = steep.exec(orig_tea);
        assert_eq!(steep.get_name(), "test_steep");
        assert_eq!(new_tea.x.unwrap(), orig_tea.x.unwrap() + 10);
    }

    #[test]
    fn create_pour_no_params() {
        let pour = Pour {
            name: String::from("test_pour"),
            computation: Box::new(|tea: Vec<TestTea>, _args| {
                tea.into_iter()
                   .map(|tea| {
                       let new_tea = tea.clone();
                       new_tea
                   })
                   .collect()
            }),
            params: None,
        };
        let orig_tea = vec![TestTea::default()];
        let orig_tea_copy = orig_tea.clone();
        let new_tea = pour.exec(orig_tea);
        assert_eq!(pour.get_name(), "test_pour");
        assert_eq!(new_tea.x, orig_tea.x);
    }

    #[test]
    fn create_pour_with_params() {
        let pour = Pour {
            name: String::from("test_pour"),
            computation: Box::new(|tea: Vec<TestTea>, args| {
                tea.into_iter()
                   .map(|tea| {
                       let new_tea = tea.clone();
                       match args {
                           None => println!("Nothing"),
                           Some(_box_args) => {
                               let _box_args = _box_args.as_any().downcast_ref::<TestArgs>().unwrap();
                           }
                       }
                       new_tea
                   })
                   .collect()
            }),
            params: Some(Box::new(TestArgs { val: 10 })),
        };
        let orig_tea = vec![TestTea::default()];
        let orig_tea_copy = orig_tea.clone();
        let new_tea = pour.exec(orig_tea);
        assert_eq!(pour.get_name(), "test_pour");
        assert_eq!(new_tea.x, orig_tea.x);
    }

    #[test]
    fn create_skim_no_params() {
        let skim = Skim {
            name: String::from("test_skim"),
            computation: Box::new(|tea: Vec<TestTea>, _args| {
                tea.into_iter()
                   .map(|mut tea| {
                       new_tea.x = None;
                       new_tea
                   })
                   .collect()
            }),
            params: None,
        };
        let orig_tea = vec![TestTea::default()];
        let new_tea = skim.exec(orig_tea);
        assert_eq!(skim.get_name(), "test_skim");
        assert_eq!(new_tea.x, None);
    }

    #[test]
    fn create_skim_with_params() {
        let skim = Skim {
            name: String::from("test_skim"),
            computation: Box::new(|tea: Vec<TestTea>, args| {
                tea.into_iter()
                   .map(|mut tea| {
                       match args {
                           None => println!("Nothing"),
                           Some(box_args) => {
                               let box_args = box_args.as_any().downcast_ref::<TestSkimArgs>().unwrap();
                               let field = box_args.field;
                               match field {
                                   "x" => new_tea.x = None,
                                   _ => panic!("unknown field")
                               };
                           }
                       }
                       new_tea
                   })
                   .collect()
            }),
            params: Some(Box::new(TestSkimArgs { field: "x" })),
        };
        let orig_tea = vec![TestTea::default()];
        let new_tea = skim.exec(orig_tea);
        assert_eq!(skim.get_name(), "test_skim");
        assert_eq!(new_tea.x, None);
    }
}
