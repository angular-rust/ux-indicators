use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct VoscFactory {
}

impl VoscFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for VoscFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}