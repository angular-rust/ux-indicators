use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct ApoFactory {
}

impl ApoFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for ApoFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}