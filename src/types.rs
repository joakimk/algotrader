use chrono::Local;
use chrono::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawBar {
    pub timestamp: u64,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: u64,
}

#[derive(Debug)]
pub struct Bar {
    pub time: DateTime<Local>,
    pub timestamp: u64,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: u64,
}

#[derive(Debug)]
pub struct Day {
    pub date : Date<Local>,
    pub open_time : DateTime<Local>,
    pub close_time : DateTime<Local>,

    pub open : f32,
    pub high : f32,
    pub low : f32,
    pub close : f32,
    //pub volume : u64,
}

#[derive(Debug)]
pub struct Chart {
    pub symbol : String,
    pub timeframe : i32,
    pub bars : Vec<Bar>,
    pub days : Vec<Day>,
}

// This is the data strategies will have to make decisions
// based on single charts or correlations.
//pub struct Watchlist {
//    pub charts : Vec<Chart>,
//}
//
//pub struct Setup {
//    pub watchlist : Watchlist,
//    pub strategies : Vec<Strategy>,
//    pub active : Bool
//}

