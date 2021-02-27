use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct LagFactory {
}

impl LagFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for LagFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}