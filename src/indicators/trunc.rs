use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct TruncFactory {
}

impl TruncFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for TruncFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}