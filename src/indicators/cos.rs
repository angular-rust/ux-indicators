use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct CosFactory {
}

impl CosFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for CosFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}