use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct StdDevFactory {
}

impl StdDevFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for StdDevFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}