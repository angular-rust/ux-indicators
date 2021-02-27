use crate::{Factory};
use crate::indicators::SimpleMovingAverage;

pub struct LinRegInterceptFactory {
}

impl LinRegInterceptFactory {
    pub fn new() -> Self {
        Self{}
    }
}

impl Factory for LinRegInterceptFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}