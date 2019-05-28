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

#[cfg(test)]
mod tests {
    use super::super::ingredient::{Fill, Steep, Pour, Argument};
    use super::super::tea::Tea;
    use super::super::source::Source;
    use std::any::Any;

    #[derive(Debug, PartialEq, Default)]
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
            computation: Box::new(|_args| {
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
            computation: Box::new(|_args| {
                TestTea::new(Box::new(TestTea::default()))
            }),
            params: Some(Box::new(TestArgs { val: 5 })),
        };
        assert_eq!(fill.get_name(), "test_fill");
    }
}
