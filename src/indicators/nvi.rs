use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct NviFactory {
}

impl NviFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for NviFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}