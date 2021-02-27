use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct WcPriceFactory {
}

impl WcPriceFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for WcPriceFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}