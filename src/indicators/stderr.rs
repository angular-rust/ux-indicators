use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct StdErrFactory {
}

impl StdErrFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for StdErrFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}