use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct KamaFactory {
}

impl KamaFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for KamaFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}