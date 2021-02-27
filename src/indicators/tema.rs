use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct TemaFactory {
}

impl TemaFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for TemaFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}