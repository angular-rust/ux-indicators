use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct EdecayFactory {
}

impl EdecayFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for EdecayFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}