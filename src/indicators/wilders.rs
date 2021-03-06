use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct WildersFactory {
}

impl WildersFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for WildersFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}