use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct AroonFactory {
}

impl AroonFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for AroonFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}