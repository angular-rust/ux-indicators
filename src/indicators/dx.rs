use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct DxFactory {
}

impl DxFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for DxFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}