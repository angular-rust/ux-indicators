use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct SqrtFactory {
}

impl SqrtFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for SqrtFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}