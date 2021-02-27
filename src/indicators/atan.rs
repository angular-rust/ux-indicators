use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct AtanFactory {
}

impl AtanFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for AtanFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}
