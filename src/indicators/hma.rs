use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct HmaFactory {
}

impl HmaFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for HmaFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}