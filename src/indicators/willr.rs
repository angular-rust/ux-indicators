use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct WillrFactory {
}

impl WillrFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for WillrFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}