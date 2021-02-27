use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct VolatilityFactory {
}

impl VolatilityFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for VolatilityFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}