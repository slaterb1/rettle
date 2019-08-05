use std::any::Any;
use std::fmt::Debug;

/// This trait must be given to the data structure(s) that will be processed by the ETL.
pub trait Tea: Send + Debug {
    /// Helper function that returns Box<dyn Tea> object as `Any`.
    ///
    /// This needs to be defined for the struct inheriting the `Tea` trait due to size not being
    /// known at compile time.
    ///
    /// # Example Method Implementation
    /// ```ignore
    /// fn as_any(&self) -> dyn Any {
    ///     self
    /// }
    /// ```
    fn as_any(&self) -> &dyn Any;
}

