use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct NatrFactory {
}

impl NatrFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for NatrFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}