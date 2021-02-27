use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct VwmaFactory {
}

impl VwmaFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for VwmaFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}