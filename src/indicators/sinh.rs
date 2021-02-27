use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct SinhFactory {
}

impl SinhFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for SinhFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}