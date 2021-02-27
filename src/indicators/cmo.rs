use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct CmoFactory {
}

impl CmoFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for CmoFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}