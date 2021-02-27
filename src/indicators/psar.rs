use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct PsarFactory {
}

impl PsarFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for PsarFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}