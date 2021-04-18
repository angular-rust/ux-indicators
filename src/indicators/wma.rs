#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt;

use rust_decimal::prelude::*;
use rust_decimal_macros::*;
use std::collections::VecDeque;

use crate::errors::{Error, ErrorKind};
use crate::{Close, Next, Reset};

use crate::indicators::SimpleMovingAverage;
use crate::Factory;

pub struct WmaFactory {}

impl WmaFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl Factory for WmaFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}

/// Weighted moving average (SMA).
///
/// # Formula
///
/// ![WMA](https://wikimedia.org/api/rest_v1/media/math/render/svg/e2bf09dc6deaf86b3607040585fac6078f9c7c89)
///
/// Where:
///
/// * _WMA<sub>t</sub>_ - value of simple moving average at a point of time _t_
/// * _period_ - number of periods (length)
/// * _p<sub>t</sub>_ - input value at a point of time _t_
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)
///

// # Example
//
// ```
// use core::indicators::WeightedMovingAverage;
// use core::Next;
//
// let mut wma = WeightedMovingAverage::new(3).unwrap();
// assert_eq!(wma.next(10.0), f64::INFINITY);
// ```
//

/// # Links
///
/// * [Weighted Moving Average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Simple_moving_average)
///

#[derive(Debug, Clone)]
pub struct WeightedMovingAverage {
    period: u32,
    index: usize,
    count: u32,
    sum: Decimal,        /* Flat sum of previous numbers. */
    weight_sum: Decimal, /* Weighted sum of previous numbers. */
    vec: VecDeque<f64>,
}

impl WeightedMovingAverage {
    // pub fn new(period: u32) -> Result<WeightedMovingAverage, Error> {
    pub fn new(period: u32) -> Result<WeightedMovingAverage, Error> {
        match period {
            // 0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let indicator = WeightedMovingAverage {
                    period,
                    index: 0,
                    count: 0,
                    sum: Decimal::zero(),
                    weight_sum: Decimal::zero(),
                    vec: VecDeque::with_capacity(period as usize),
                };
                Ok(indicator)
            }
        }
    }
}

impl Next<f64> for WeightedMovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        // self.index = (self.index + 1) % (self.period as usize);

        // let old_val = self.vec[self.index];
        // self.vec[self.index] = input;

        // fill windoe upto period
        if self.count < self.period - 1 {
            self.count += 1;
            self.vec.push_back(input);
            self.weight_sum +=
                Decimal::from_f64(input).unwrap() * Decimal::from_u32(self.count).unwrap();
            self.sum += Decimal::from_f64(input).unwrap();
            return f64::INFINITY;
        }

        let weights: Decimal = Decimal::from_u32(self.period).unwrap()
            * (Decimal::from_u32(self.period).unwrap() + Decimal::from_u32(1).unwrap())
            / Decimal::from_u32(2).unwrap();

        // sliding window
        self.weight_sum +=
            Decimal::from_f64(input).unwrap() * Decimal::from_u32(self.period).unwrap();
        self.sum += Decimal::from_f64(input).unwrap();

        let output: Decimal = self.weight_sum / weights;

        self.vec.push_back(input);
        self.weight_sum -= self.sum;
        self.sum -= Decimal::from_f64(self.vec.pop_front().unwrap()).unwrap();

        output
            .round_dp_with_strategy(3, RoundingStrategy::RoundHalfUp)
            .to_f64()
            .unwrap()
    }
}

impl<'a, T: Close> Next<&'a T> for WeightedMovingAverage {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for WeightedMovingAverage {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.sum = Decimal::from_f64(0.0).unwrap();
        for idx in 0..(self.period as usize) {
            self.vec[idx] = 0.0;
        }
    }
}

impl Default for WeightedMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for WeightedMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WMA({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    // test_indicator!(WeightedMovingAverage);

    #[test]
    fn test_new() {
        initialize();
        // assert!(WeightedMovingAverage::new(0).is_err());
        assert!(WeightedMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        initialize();
        let _params = &TESTS["wma"];
        println!();
        for _test in _params.tests.iter() {
            let _period = _test.options[0];
            let _input = &_test.inputs[0];
            let _output = &_test.outputs[0];

            println!("WMA WITH PERIOD {}", _period);
            let mut indicator = WeightedMovingAverage::new(_period as u32).unwrap();
            for val in _input.iter() {
                let res = indicator.next(*val);
                println!("INPUT {} OUTPUT {}", val, res);
            }
        }
        // let mut indicator = WeightedMovingAverage::new(4).unwrap();
        // assert_eq!(indicator.next(4.0), 4.0);
        // assert_eq!(indicator.next(5.0), 4.5);
        // assert_eq!(indicator.next(6.0), 5.0);
        // assert_eq!(indicator.next(6.0), 5.25);
        // assert_eq!(indicator.next(6.0), 5.75);
        // assert_eq!(indicator.next(6.0), 6.0);
        // assert_eq!(indicator.next(2.0), 5.0);
    }

    // #[test]
    // fn test_next_with_bars() {
    //     fn bar(close: f64) -> Bar {
    //         Bar::new().close(close)
    //     }

    //     let mut indicator = WeightedMovingAverage::new(3).unwrap();
    //     assert_eq!(indicator.next(&bar(4.0)), 4.0);
    //     assert_eq!(indicator.next(&bar(4.0)), 4.0);
    //     assert_eq!(indicator.next(&bar(7.0)), 5.0);
    //     assert_eq!(indicator.next(&bar(1.0)), 4.0);
    // }

    // #[test]
    // fn test_reset() {
    //     let mut indicator = WeightedMovingAverage::new(4).unwrap();
    //     assert_eq!(indicator.next(4.0), 4.0);
    //     assert_eq!(indicator.next(5.0), 4.5);
    //     assert_eq!(indicator.next(6.0), 5.0);

    //     indicator.reset();
    //     assert_eq!(indicator.next(99.0), 99.0);
    // }

    #[test]
    fn test_default() {
        WeightedMovingAverage::default();
    }

    #[test]
    fn test_display() {
        let indicator = WeightedMovingAverage::new(5).unwrap();
        assert_eq!(format!("{}", indicator), "WMA(5)");
    }
}
