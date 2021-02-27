use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct MswFactory {
}

impl MswFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for MswFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}