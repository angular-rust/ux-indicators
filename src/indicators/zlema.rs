use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct ZlemaFactory {
}

impl ZlemaFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for ZlemaFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}