use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct VhfFactory {
}

impl VhfFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for VhfFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}