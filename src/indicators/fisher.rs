use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct FisherFactory {
}

impl FisherFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for FisherFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}