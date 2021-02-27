use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct AroonOscFactory {
}

impl AroonOscFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for AroonOscFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}