use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct RoundFactory {
}

impl RoundFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for RoundFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}