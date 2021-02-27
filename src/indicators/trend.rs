use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct TrendFactory {
}

impl TrendFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for TrendFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}