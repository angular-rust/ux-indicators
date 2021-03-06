use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct DecayFactory {
}

impl DecayFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for DecayFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}