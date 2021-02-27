use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct TrueRangeFactory {
}

impl TrueRangeFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for TrueRangeFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}