use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct PviFactory {
}

impl PviFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for PviFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}