use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct MomFactory {
}

impl MomFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for MomFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}