use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct CeilFactory {
}

impl CeilFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for CeilFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}