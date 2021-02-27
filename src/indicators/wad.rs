use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct WadFactory {
}

impl WadFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for WadFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}