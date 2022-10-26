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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug)]
pub struct DayResult {
    // The change we arrived at once all positions where sold.
    pub close_percent : f32,

    // Shows if there was potential for better exits.
    pub high_percent : f32,

    // How much we risked during the day.
    pub low_percent : f32,

    pub timestamp : u64,

    pub trades : Vec<Trade>
}

#[derive(Debug)]
pub struct Trade {
    pub symbol : String,
    pub buy_time : DateTime<Local>,
    pub sell_time : Option<DateTime<Local>>,
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

