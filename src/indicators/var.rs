use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct VarFactory {
}

impl VarFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for VarFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}