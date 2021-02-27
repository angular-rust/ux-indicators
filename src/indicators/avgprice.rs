use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct AvgPriceFactory {
}

impl AvgPriceFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for AvgPriceFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}
