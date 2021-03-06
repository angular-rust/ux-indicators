use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct TsfFactory {
}

impl TsfFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for TsfFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}