use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct ToRadFactory {
}

impl ToRadFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for ToRadFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}