use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct StochFactory {
}

impl StochFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for StochFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}