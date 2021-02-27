use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct AddFactory {
}

impl AddFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for AddFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}