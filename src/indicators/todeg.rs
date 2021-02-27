use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct ToDegFactory {
}

impl ToDegFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for ToDegFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}