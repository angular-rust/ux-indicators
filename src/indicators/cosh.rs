use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct CoshFactory {
}

impl CoshFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for CoshFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}