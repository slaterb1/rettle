use std::any::Any;

pub trait Tea {
    fn as_any(&self) -> &dyn Any;
    fn new(self: Box<Self>) -> Box<dyn Tea>;
}

