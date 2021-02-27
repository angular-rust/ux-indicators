use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct AoFactory {
}

impl AoFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for AoFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}