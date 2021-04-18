#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt;

use crate::errors::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::errors::{Error, ErrorKind};
use crate::{Close, Next, Reset};
use crate::{Frame, Slot, SlotPtr, SlotType};

use crate::Factory;

pub struct SmaFactory {}

impl SmaFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl Factory for SmaFactory {
    fn create() -> Box<dyn Next<f64, Output = Box<[f64]>>> {
        Box::new(SimpleMovingAverage::default())
    }
}

/// Simple moving average (SMA).
///
/// # Formula
///
/// ![SMA](https://wikimedia.org/api/rest_v1/media/math/render/svg/e2bf09dc6deaf86b3607040585fac6078f9c7c89)
///
/// Where:
///
/// * _SMA<sub>t</sub>_ - value of simple moving average at a point of time _t_
/// * _n_ - number of periods (length)
/// * _p<sub>t</sub>_ - input value at a point of time _t_
///
/// # Parameters
///
/// * _n_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// // use core::indicators::SimpleMovingAverage;
/// // use core::Next;
///
/// // let mut sma = SimpleMovingAverage::new(3).unwrap();
/// // assert_eq!(sma.next(10.0), 10.0);
/// // assert_eq!(sma.next(11.0), 10.5);
/// // assert_eq!(sma.next(12.0), 11.0);
/// // assert_eq!(sma.next(13.0), 12.0);
/// ```
///
/// # Links
///
/// * [Simple Moving Average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Simple_moving_average)
///

#[derive(Debug, Clone)]
pub struct SimpleMovingAverage<'a> {
    period: u32,
    index: usize,
    count: u32,
    sum: f64,
    vec: Vec<f64>,
    pub inputs: HashMap<&'a str, Rc<RefCell<Slot>>>,
    pub outputs: HashMap<&'a str, Rc<RefCell<Slot>>>,
}

impl<'a> SimpleMovingAverage<'a> {
    pub fn new(period: u32) -> Result<Self> {
        match period {
            // 0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let indicator = Self {
                    period,
                    index: 0,
                    count: 0,
                    sum: 0.0,
                    vec: vec![0.0; period as usize],
                    inputs: [("input", Rc::new(RefCell::new(Slot::new(SlotType::Input))))]
                        .iter()
                        .cloned()
                        .collect(),
                    outputs: [("output", Rc::new(RefCell::new(Slot::new(SlotType::Output))))]
                        .iter()
                        .cloned()
                        .collect(),
                };

                Ok(indicator)
            }
        }
    }

    // fn slot(&mut self, name: &str) -> Option<&mut Rc<RefCell<Slot>>> {
    //     self.inputs.get_mut(name)
    // }

    fn slot(&mut self, name: &str) -> Option<&Rc<RefCell<Slot>>> {
        self.inputs.get(name)
    }

    fn process(&mut self) {
        match self.inputs.get("input") {
            None => println!("INPUTS OOPS {:?}", self),
            Some(slot) => {
                // println!("HERE {:?}", slot.borrow_mut());

                let input = slot.borrow_mut().get();

                self.index = (self.index + 1) % (self.period as usize);
                let old_val = self.vec[self.index];

                self.vec[self.index] = input;

                // fill counter upto period
                if self.count < self.period {
                    self.count += 1;
                }

                // sliding window
                self.sum = self.sum - old_val + input;
                let output = self.sum / (self.count as f64);
                match self.outputs.get("output") {
                    None => println!("OUTPUT OOPS {:?}", self),
                    Some(slot) => {
                        slot.borrow_mut().put(output);
                    }
                }
            }
        }
    }
}

impl<'a> Next<f64> for SimpleMovingAverage<'a> {
    type Output = Box<[f64]>;

    fn next(&mut self, input: f64) -> Self::Output {
        self.index = (self.index + 1) % (self.period as usize);

        let old_val = self.vec[self.index];
        self.vec[self.index] = input;

        // fill counter upto period
        if self.count < self.period {
            self.count += 1;
        }

        // sliding window
        self.sum = self.sum - old_val + input;
        Box::new([self.sum / (self.count as f64)])
    }
}

// impl<'a, T: Close> Next<&'a T> for SimpleMovingAverage {
//     type Output = f64;

//     fn next(&mut self, input: &'a T) -> Self::Output {
//         self.next(input.close())
//     }
// }

impl<'a> Reset for SimpleMovingAverage<'a> {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.sum = 0.0;
        for idx in 0..(self.period as usize) {
            self.vec[idx] = 0.0;
        }
    }
}

impl<'a> Default for SimpleMovingAverage<'a> {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl<'a> fmt::Display for SimpleMovingAverage<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMA({})", self.period)
    }
}

// fn run_test<T>(test: T) -> ()
//     where T: FnOnce() -> () + panic::UnwindSafe
// {
//     setup();

//     let result = panic::catch_unwind(|| {
//         test()
//     });

//     teardown();

//     assert!(result.is_ok())
// }

// #[test]
// fn test() {
//     run_test(|| {
//         let ret_value = function_under_test();
//         assert!(ret_value);
//     })
// }

// #[test]
// fn test_something_interesting() {
//     run_test(|| {
//         let true_or_false = do_the_test();

//         assert!(true_or_false);
//     })
// }
// fn run_test<T>(test: T) -> ()
//     where T: FnOnce() -> () + panic::UnwindSafe
// {
//     setup();

//     let result = panic::catch_unwind(|| {
//         test()
//     });

//     teardown();

//     assert!(result.is_ok())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;
    use crate::{Frame, Slot, SlotPtr, SlotType};

    // test_indicator!(SimpleMovingAverage);

    #[test]
    fn test_new() {
        initialize();
        // assert!(SimpleMovingAverage::new(0).is_err());
        assert!(SimpleMovingAverage::new(1).is_ok());
    }

    fn step(sma: &mut SimpleMovingAverage, val: f64) -> f64 {
        let input = sma.slot("input").unwrap();
        input.borrow_mut().put(val);
        sma.process();
        let output = sma.outputs.get("output").unwrap();
        output.borrow_mut().get()
    }

    #[test]
    fn test_process() {
        initialize();

        // assert!(SimpleMovingAverage::new(0).is_err());
        // assert!(SimpleMovingAverage::new(1).is_ok());

        let mut sma = SimpleMovingAverage::new(4).unwrap();
        // match sma.slot("input") {
        //     None => println!("WIRE1 NOT CONNECTED FOR input"),
        //     Some(input) => {
        //         // let mut input = input.borrow_mut();
        //         input.borrow_mut().put(1.0);
        //         sma.process();
        //         input.borrow_mut().put(1.5);
        //         sma.process();
        //         input.borrow_mut().put(1.0);
        //         sma.process();
        //     }
        // }
        assert_eq!(step(&mut sma, 4.0), 4.0);
        assert_eq!(step(&mut sma, 5.0), 4.5);
        assert_eq!(step(&mut sma, 6.0), 5.0);
        assert_eq!(step(&mut sma, 6.0), 5.25);
        assert_eq!(step(&mut sma, 6.0), 5.75);
        assert_eq!(step(&mut sma, 6.0), 6.0);
        assert_eq!(step(&mut sma, 2.0), 5.0);
    }

    // let mut inputs: HashMap<&str, SlotPtr> = HashMap::new();
    // // let mut new_tail = Box::new(Slot::new(SlotType::Input));
    // // let ptr = &mut *new_tail;
    // // inputs.insert("input", ptr);
    // // inputs.insert("input", &mut *new_tail);
    // inputs.insert("input", &mut *Box::new(Slot::new(SlotType::Input)));

    // println!("INPUTS {:?}", inputs);

    // let slot = *inputs.get("input").unwrap();
    // unsafe {
    //     let slot = &mut *slot;
    //     slot.put(2.0);
    // }

    // unsafe {
    //     let slot = &mut *slot;
    //     println!("SLOT {:?}", slot);
    // }

    // let a: HashMap<&str, SlotPtr> = [("input", ptr)].iter().cloned().collect();

    // #[test]
    // fn test_next() {
    //     initialize();

    //     let mut sma = SimpleMovingAverage::new(4).unwrap();
    //     assert_eq!(sma.next(4.0), 4.0);
    //     assert_eq!(sma.next(5.0), 4.5);
    //     assert_eq!(sma.next(6.0), 5.0);
    //     assert_eq!(sma.next(6.0), 5.25);
    //     assert_eq!(sma.next(6.0), 5.75);
    //     assert_eq!(sma.next(6.0), 6.0);
    //     assert_eq!(sma.next(2.0), 5.0);
    // }

    // #[test]
    // fn test_next_with_bars() {
    //     initialize();

    //     fn bar(close: f64) -> Bar {
    //         Bar::new().close(close)
    //     }

    //     let mut sma = SimpleMovingAverage::new(3).unwrap();
    //     assert_eq!(sma.next(&bar(4.0)), 4.0);
    //     assert_eq!(sma.next(&bar(4.0)), 4.0);
    //     assert_eq!(sma.next(&bar(7.0)), 5.0);
    //     assert_eq!(sma.next(&bar(1.0)), 4.0);
    // }

    // #[test]
    // fn test_reset() {
    //     initialize();

    //     let mut sma = SimpleMovingAverage::new(4).unwrap();
    //     assert_eq!(sma.next(4.0), 4.0);
    //     assert_eq!(sma.next(5.0), 4.5);
    //     assert_eq!(sma.next(6.0), 5.0);

    //     sma.reset();
    //     assert_eq!(sma.next(99.0), 99.0);
    // }

    #[test]
    fn test_default() {
        initialize();

        SimpleMovingAverage::default();
    }

    #[test]
    fn test_display() {
        initialize();

        let sma = SimpleMovingAverage::new(5).unwrap();
        assert_eq!(format!("{}", sma), "SMA(5)");
    }
}
