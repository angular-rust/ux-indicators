use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct TypPriceFactory {
}

impl TypPriceFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for TypPriceFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}