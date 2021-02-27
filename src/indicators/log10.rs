use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct Log10Factory {
}

impl Log10Factory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for Log10Factory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}