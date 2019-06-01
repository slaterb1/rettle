use std::any::Any;
use super::brewer::Brewery;

pub use super::tea::Tea;

pub trait Ingredient<'a> {
    fn exec(&self, tea: Box<dyn Tea>) -> Box<dyn Tea>;
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
    pub computation: Box<Fn(&Option<Box<dyn Argument>>, &Brewery, &Vec<Box<dyn Ingredient>>)>,
    pub params: Option<Box<dyn Argument>>,
}

pub struct Transfuse;

pub struct Steep {
    pub name: String,
    pub computation: Box<Fn(Box<dyn Tea>, &Option<Box<dyn Argument>>) -> Box<dyn Tea>>, 
    pub params: Option<Box<dyn Argument>>,
}

pub struct Skim {
    pub name: String,
    pub computation: Box<Fn(Box<dyn Tea>, &Option<Box<dyn Argument>>) -> Box<dyn Tea>>, 
    pub params: Option<Box<dyn Argument>>,
}

pub struct Pour{
    pub name: String,
    pub computation: Box<Fn(Box<dyn Tea>, &Option<Box<dyn Argument>>) -> Box<dyn Tea>>, 
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
    fn exec(&self, tea: Box<dyn Tea>) -> Box<dyn Tea> {
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
    fn exec(&self, tea: Box<dyn Tea>) -> Box<dyn Tea> {
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
    fn exec(&self, tea: Box<dyn Tea>) -> Box<dyn Tea> {
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
        fn new(self: Box<Self>) -> Box<dyn Tea> {
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
            computation: Box::new(|_args: &Option<Box<dyn Argument>>| {
                TestTea::new(Box::new(TestTea::default()))
            }),
            params: None,
        };
        assert_eq!(fill.get_name(), "test_fill");
    }

    #[test]
    fn create_fill_with_params() {
        let fill = Fill {
            name: String::from("test_fill"),
            source: String::from("text"),
            computation: Box::new(|_args: &Option<Box<dyn Argument>>| {
                TestTea::new(Box::new(TestTea::default()))
            }),
            params: Some(Box::new(TestArgs { val: 5 })),
        };
        assert_eq!(fill.get_name(), "test_fill");
    }

    #[test]
    fn create_steep_no_params() {
        let steep = Steep {
            name: String::from("test_steep"),
            computation: Box::new(|tea: &Box<dyn Tea>, _args: &Option<Box<dyn Argument>>| {
                let tea = tea.as_any().downcast_ref::<TestTea>().unwrap();
                let mut new_tea = tea.clone();
                new_tea.x = tea.x + 5;
                Box::new(new_tea)
            }),
            params: None,
        };
        let orig_tea = TestTea::new(Box::new(TestTea::default()));
        let new_tea = steep.exec(&orig_tea);
        let orig_tea = orig_tea.as_any().downcast_ref::<TestTea>().unwrap();
        let new_tea = new_tea.as_any().downcast_ref::<TestTea>().unwrap();
        assert_eq!(steep.get_name(), "test_steep");
        assert_eq!(new_tea.x, orig_tea.x + 5);
    }

    #[test]
    fn create_steep_with_params() {
        let steep = Steep {
            name: String::from("test_steep"),
            computation: Box::new(|tea: &Box<dyn Tea>, args: &Option<Box<dyn Argument>>| {
                let tea = tea.as_any().downcast_ref::<TestTea>().unwrap();
                let mut new_tea = tea.clone();
                match args {
                    None => println!("Nothing"),
                    Some(box_args) => {
                        let box_args = box_args.as_any().downcast_ref::<TestArgs>().unwrap();
                        new_tea.x = tea.x + box_args.val;
                    }
                }
                Box::new(new_tea)
            }),
            params: Some(Box::new(TestArgs { val: 10 })),
        };
        let orig_tea = TestTea::new(Box::new(TestTea::default()));
        let new_tea = steep.exec(&orig_tea);
        let orig_tea = orig_tea.as_any().downcast_ref::<TestTea>().unwrap();
        let new_tea = new_tea.as_any().downcast_ref::<TestTea>().unwrap();
        assert_eq!(steep.get_name(), "test_steep");
        assert_eq!(new_tea.x, orig_tea.x + 10);
    }

    #[test]
    fn create_pour_no_params() {
        let pour = Pour {
            name: String::from("test_pour"),
            computation: Box::new(|tea: &Box<dyn Tea>, _args: &Option<Box<dyn Argument>>| {
                let tea = tea.as_any().downcast_ref::<TestTea>().unwrap();
                let new_tea = tea.clone();
                println!("Output tea to terminal: {:?}", tea);
                Box::new(new_tea)
            }),
            params: None,
        };
        let orig_tea = TestTea::new(Box::new(TestTea::default()));
        let new_tea = pour.exec(&orig_tea);
        let orig_tea = orig_tea.as_any().downcast_ref::<TestTea>().unwrap();
        let new_tea = new_tea.as_any().downcast_ref::<TestTea>().unwrap();
        assert_eq!(pour.get_name(), "test_pour");
        assert_eq!(new_tea.x, orig_tea.x);
    }

    #[test]
    fn create_pour_with_params() {
        let pour = Pour {
            name: String::from("test_pour"),
            computation: Box::new(|tea: &Box<dyn Tea>, args: &Option<Box<dyn Argument>>| {
                let tea = tea.as_any().downcast_ref::<TestTea>().unwrap();
                let new_tea = tea.clone();
                match args {
                    None => panic!("No params!"),
                    Some(box_args) => {
                        let box_args = box_args.as_any().downcast_ref::<TestArgs>().unwrap();
                        println!("Output tea to terminal, with param: {:?} {}", tea, box_args.val);
                    }
                }
                Box::new(new_tea)
            }),
            params: Some(Box::new(TestArgs { val: 10 })),
        };
        let orig_tea = TestTea::new(Box::new(TestTea::default()));
        let new_tea = pour.exec(&orig_tea);
        let orig_tea = orig_tea.as_any().downcast_ref::<TestTea>().unwrap();
        let new_tea = new_tea.as_any().downcast_ref::<TestTea>().unwrap();
        assert_eq!(pour.get_name(), "test_pour");
        assert_eq!(new_tea.x, orig_tea.x);
    }
}
