use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct DemaFactory {
}

impl DemaFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for DemaFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}