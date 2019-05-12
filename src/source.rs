use std::any::Any;

pub use super::tea::{Tea, RawTea1};
pub use super::ingredient::Fill;

///
/// Raw data inputs
pub trait Source {
    ///
    /// Currently this outputs Tea, in the future it will pull in all desired data, pushing it in
    /// batches to a source that the Brewers pull from.
    fn collect(&self) -> Tea;
    fn as_any(&self) -> &dyn Any;
    fn print(&self);
}

impl Source for Fill {
    fn collect(&self) -> Tea {
        let data = r#"{
            "x": 10000,
            "str_val": "new_values",
            "y": false
        }"#;
        let data: RawTea1 = serde_json::from_str(data).unwrap();
        Tea { data }
    }
    fn print(&self) {
        println!("Current Source: {}", self.name);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


