use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct StochRsiFactory {
}

impl StochRsiFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for StochRsiFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}