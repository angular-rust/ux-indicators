use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct LnFactory {
}

impl LnFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for LnFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}