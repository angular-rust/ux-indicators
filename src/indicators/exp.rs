use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct ExpFactory {
}

impl ExpFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for ExpFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}