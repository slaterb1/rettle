use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// Resulting data that is being manipulated in the brew.
pub struct Tea {
    pub x: i32,
    pub str_val: String,
    pub y: bool,
}

impl Tea {
    /// Temporarily, new creates sample data to test constructing the recipe and adding it to the
    /// Pot. In the future, Fill will result in data that is passed on to the processes to be
    /// brewed.
    pub fn new() -> Tea {
        let data = r#"{
          "x": 1,
          "str_val": "new_values",
          "y": false
        }"#;
        let data: Tea = serde_json::from_str(data).unwrap();
        data
    }
}


