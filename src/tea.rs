use std::any::Any;

/// This trait must be given to the data structure(s) that will be processed by the ETL.
pub trait Tea {
    /// Helper function that returns Box<dyn Tea> object as `Any`.
    ///
    /// This needs to be defined for the struct inheriting the `Tea` trait due to size not being
    /// known at compile time.
    ///
    /// # Example Method Implementation
    ///
    /// fn as_any(&self) -> dyn Any {
    ///     self
    /// }
    fn as_any(&self) -> &dyn Any;

    /// Function definition for creating a new struct "Tea" object used by the `Pot::brew()` method. 
    ///
    /// This needs to be created by the Developer to specify how the data coming from the `Fill`
    /// operation will be structured and initialized before being manipulated by the rest of the
    /// `Pot::recipe` steps via the `Brewer::make_tea` method.
    fn new(self: Box<Self>) -> Box<dyn Tea>;
}

