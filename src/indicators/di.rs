use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct DiFactory {
}

impl DiFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for DiFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}