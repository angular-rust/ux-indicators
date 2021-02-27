use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct DmFactory {
}

impl DmFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for DmFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}