use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct TanFactory {
}

impl TanFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for TanFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}