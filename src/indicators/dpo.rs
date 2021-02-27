use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct DpoFactory {
}

impl DpoFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for DpoFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}