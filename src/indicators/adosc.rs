use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct AdOscFactory {
}

impl AdOscFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for AdOscFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}