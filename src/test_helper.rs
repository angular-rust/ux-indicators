use std::sync::Once;
use std::fs;
use std::collections::HashMap;
use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};
// use serde_json::{Map, Value};

use super::{Close, High, Low, Open, Volume};

#[derive(Debug, PartialEq)]
pub struct Bar {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            open: 0.0,
            close: 0.0,
            low: 0.0,
            high: 0.0,
            volume: 0.0,
        }
    }

    //pub fn open<T: Into<f64>>(mut self, val :T ) -> Self {
    //    self.open = val.into();
    //    self
    //}

    pub fn high<T: Into<f64>>(mut self, val: T) -> Self {
        self.high = val.into();
        self
    }

    pub fn low<T: Into<f64>>(mut self, val: T) -> Self {
        self.low = val.into();
        self
    }

    pub fn close<T: Into<f64>>(mut self, val: T) -> Self {
        self.close = val.into();
        self
    }

    pub fn volume(mut self, val: f64) -> Self {
        self.volume = val;
        self
    }
}

impl Open for Bar {
    fn open(&self) -> f64 {
        self.open
    }
}

impl Close for Bar {
    fn close(&self) -> f64 {
        self.close
    }
}

impl Low for Bar {
    fn low(&self) -> f64 {
        self.low
    }
}

impl High for Bar {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Volume for Bar {
    fn volume(&self) -> f64 {
        self.volume
    }
}

pub fn round(num: f64) -> f64 {
    (num * 1000.0).round() / 1000.00
}

// macro_rules! test_indicator {
//     ($i:tt) => {
//         #[test]
//         fn test_indicator() {
//             initialize();

//             let bar = Bar::new();

//             // ensure Default trait is implemented
//             let mut indicator = $i::default();

//             // ensure Next<f64> is implemented
//             let first_output = indicator.next(12.3);

//             // ensure next accepts &DataItem as well
//             indicator.next(&bar);

//             // ensure Reset is implemented and works correctly
//             indicator.reset();
//             assert_eq!(indicator.next(12.3), first_output);

//             // ensure Display is implemented
//             format!("{}", indicator);
//         }
//     };
// }

// cargo test -- --nocapture

static INIT: Once = Once::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    pub input: u32,
    pub options: u32,
    pub output: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Test {
    pub options: Vec<f64>,
    pub inputs: Vec<Vec<f64>>,
    pub outputs: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestIndicator {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub indicatortype: String,
    pub slots: Slot,
    pub options: String,
    pub tests: Vec<Test>,
}

lazy_static! {
    pub static ref TESTS: HashMap<String, TestIndicator> = {
        // println!(":::::::::::::::::::::::: INIT ::::::::::::::::::::::::");
        let contents = fs::read_to_string("tests/data.json")
            .expect("Something went wrong reading the file");

        serde_json::from_str(&contents).unwrap()
    };
}

pub fn initialize() {
    INIT.call_once(|| {
        // initialization code here
        if TESTS.len() == 0 {
            println!("Tests uninitialized")
            // println!("With text: {}", TESTS["abs"].name);
        }
    });
}