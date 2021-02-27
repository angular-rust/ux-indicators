use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct BopFactory {
}

impl BopFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for BopFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}