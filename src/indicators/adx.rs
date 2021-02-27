use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct AdxFactory {
}

impl AdxFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for AdxFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}