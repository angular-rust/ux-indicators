use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct AcosFactory {
}

impl AcosFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for AcosFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}