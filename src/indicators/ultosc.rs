use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct UltOscFactory {
}

impl UltOscFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for UltOscFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}