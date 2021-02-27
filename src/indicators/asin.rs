use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct AsinFactory {
}

impl AsinFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for AsinFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}