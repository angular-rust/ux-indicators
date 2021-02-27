use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct SumFactory {
}

impl SumFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for SumFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}