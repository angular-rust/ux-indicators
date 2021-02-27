use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct MassFactory {
}

impl MassFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for MassFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}