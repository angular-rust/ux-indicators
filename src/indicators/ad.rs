use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct AdFactory {
}

impl AdFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for AdFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}