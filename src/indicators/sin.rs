use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct SinFactory {
}

impl SinFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for SinFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}