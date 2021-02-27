use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct CciFactory {
}

impl CciFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for CciFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}