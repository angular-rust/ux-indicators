use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct MulFactory {
}

impl MulFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for MulFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}