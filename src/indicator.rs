#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::indicators::SimpleMovingAverage;
use crate::Next;

use crate::errors::Error;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum DataPoint {
    Ohlcv(Ohlcv),
    BidAsk(BidAsk),
    Frame(Frame),
}

// used for input node
#[derive(Debug, Clone)]
pub struct Ohlcv {
    // for syncing
    timestamp: u64,
    // values
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl Ohlcv {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BidAsk {
    // for syncing
    timestamp: u64,
    // price
    price: f64,
    // value
    amount: f64,
}

impl BidAsk {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            price: 0.0,
            amount: 0.0,
        }
    }
}

// is dataframe
#[derive(Debug, Clone)]
pub struct Frame {
    // for syncing
    timestamp: u64,
    // value
    data: f64,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            data: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SlotType {
    Input,
    Output,
}

// define slot
#[derive(Debug, Clone)]
pub struct Slot {
    pub name: String,
    slot_type: SlotType,
    state: f64,                          // used to store value when it input type
    pub changed: bool,                   // when it input type,
    connected: bool,                     // when in input type,
    connections: Vec<Rc<RefCell<Slot>>>, // used to keep references when it output type
}

impl Slot {
    pub fn new(slot_type: SlotType) -> Self {
        Self {
            name: String::from("slot"),
            slot_type,
            state: 0.0,
            changed: false,
            connected: false,
            connections: vec![],
        }
    }
    // output is multiple, input is single
    pub fn connect(&mut self, wire: Rc<RefCell<Slot>>) -> Result<(), Error> {
        if self.slot_type == SlotType::Output {
            return Ok(());
        }
        // self.wire = Some(wire)
        Err("The error message".into())
    }
    // pub fn wire(&self) -> Option<Rc<RefCell<Wire>>> {
    //     // &self.wire
    //     None
    // }

    // put data into buffer
    pub fn put(&mut self, val: f64) {
        if self.state != val {
            self.changed = true;
        }
        self.state = val;
    }

    // fetch data from buffer
    pub fn get(&mut self) -> f64 {
        self.changed = false;
        self.state
    }
}

pub type SlotPtr = *mut Slot;

#[derive(Debug, Clone)]
pub struct Indicator {
    inputs: HashMap<String, Slot>,
    outputs: HashMap<String, Slot>,
}

impl Indicator {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }
    pub fn slot(&self, name: &str) -> Option<Rc<RefCell<Slot>>> {
        None
    }
    pub fn stuff(&self) {}
}

#[derive(Debug, Clone)]
pub enum InputType {
    Ohlcv,
    BidAsk,
    Frame,
}

#[derive(Debug, Clone)]
pub struct Input {
    input_type: InputType,
    slots: Vec<Slot>,
    timeseries: VecDeque<DataPoint>,
}

impl Input {
    pub fn new(input_type: InputType) -> Self {
        Self {
            input_type,
            slots: vec![],
            timeseries: VecDeque::new(),
        }
    }
    pub fn slot(&self, name: &str) -> Option<Rc<RefCell<Slot>>> {
        None
    }

    pub fn push(&mut self, item: DataPoint) {
        match item {
            DataPoint::Ohlcv(_) => {
                // let a: Ohlcv = val;
                self.timeseries.push_back(item);
            }
            _ => {
                print!("unhandled node type")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct View {
    plots: Vec<Rc<Plot>>,
}

impl View {
    pub fn new() -> Self {
        Self { plots: vec![] }
    }

    pub fn attach(&mut self, plot: Rc<Plot>) {
        self.plots.push(plot);
    }
}

#[derive(Debug, Clone)]
pub struct Plot {
    view: Option<View>,
    timeseries: VecDeque<Frame>,
}

impl Plot {
    pub fn new() -> Self {
        Self {
            view: None,
            timeseries: VecDeque::new(),
        }
    }
    // draw to view
    pub fn draw(&self) {}

    pub fn slot(&self, name: &str) -> Option<Rc<RefCell<Slot>>> {
        None
    }
}

enum Node {
    // Wire(Rc<RefCell<Wire>>),
    Input(Rc<Input>),
    Indicator(Rc<Indicator>),
    Plot(Rc<Plot>),
    View(Rc<View>),
    Text(String),
}

pub fn example() {
    let mut input: Input = Input::new(InputType::Ohlcv);

    let indicator: Indicator = Indicator::new();

    let plot: Plot = Plot::new();

    // let input_wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new()));
    // let output_wire: Rc<RefCell<Wire>> = Rc::new(RefCell::new(Wire::new()));
    let mut view: View = View::new();

    // input -> indicator
    let input_close = input.slot("close").unwrap();
    let mut input_close = input_close.borrow_mut();
    {
        let indicator_close = indicator.slot("close").unwrap();
        let _ = input_close.connect(Rc::clone(&indicator_close));
    }

    // // indicator -> plot
    let indicator_output = indicator.slot("output").unwrap();
    let mut indicator_output = indicator_output.borrow_mut();
    {
        let plot_close = plot.slot("close").unwrap();
        let _ = indicator_output.connect(Rc::clone(&plot_close));
    }

    let rcplot = Rc::new(plot);
    // put plot into view
    view.attach(Rc::clone(&rcplot));

    let a: &Plot = rcplot.as_ref();
    a.draw();

    input.push(DataPoint::Ohlcv(Ohlcv {
        timestamp: 1,
        open: 0.1,
        high: 0.1,
        low: 0.1,
        close: 0.1,
        volume: 0.1,
    }));

    // collect all to workspace
    let workspace: Vec<Node> = vec![
        Node::Input(Rc::new(input)),
        Node::Indicator(Rc::new(indicator)),
        Node::Plot(rcplot),
        Node::View(Rc::new(view)),
    ];

    for item in workspace.iter() {
        match item {
            Node::View(val) => {
                let a: &View = val;
            }
            Node::Plot(val) => {
                let a: &Plot = val;
            }
            Node::Text(val) => {
                let a: String = val.to_string();
            }
            _ => {
                print!("unhandled node type")
            }
        }
    }
}

// pub struct Inticator {
//     _impl: Box<dyn Next<f64, Output = Box<[f64]>>>,
// }

// impl Inticator {
//     pub fn new(name: &str) -> Result<Inticator, JsValue> {
//         match name {
//             // 0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
//             "ichimoku" => {
//                 Ok(Inticator {
//                     _impl: Box::new(SimpleMovingAverage::default()),
//                 })
//             }
//             _ => {
//                 Ok(Inticator {
//                     _impl: Box::new(SimpleMovingAverage::default()),
//                 })
//             }
//         }
//     }

//     pub fn get_contents(&self) -> u32 {
//         0
//     }
// }
