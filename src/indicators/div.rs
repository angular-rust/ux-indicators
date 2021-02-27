use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct DivFactory {
}

impl DivFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for DivFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}