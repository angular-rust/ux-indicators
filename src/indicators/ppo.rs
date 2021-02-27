use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct PpoFactory {
}

impl PpoFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for PpoFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}