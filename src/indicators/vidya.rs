use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct VidyaFactory {
}

impl VidyaFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for VidyaFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}