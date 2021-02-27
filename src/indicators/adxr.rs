use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct AdxrFactory {
}

impl AdxrFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for AdxrFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}