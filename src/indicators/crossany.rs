use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct CrossAnyFactory {
}

impl CrossAnyFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for CrossAnyFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}