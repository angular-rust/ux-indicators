use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct MedPriceFactory {
}

impl MedPriceFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for MedPriceFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}