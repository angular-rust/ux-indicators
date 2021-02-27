use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct FoscFactory {
}

impl FoscFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for FoscFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}