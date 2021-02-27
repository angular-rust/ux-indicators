use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct MdFactory {
}

impl MdFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for MdFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}