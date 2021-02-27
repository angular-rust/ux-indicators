use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct CrossOverFactory {
}

impl CrossOverFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for CrossOverFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}