use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct TrimaFactory {
}

impl TrimaFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for TrimaFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}