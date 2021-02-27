use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct TanhFactory {
}

impl TanhFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for TanhFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}