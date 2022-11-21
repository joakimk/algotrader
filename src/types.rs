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
    pub timestamp : u64,
    pub time : DateTime<Local>,

    pub trades : Vec<Trade>,

    pub fee_amount : f32,

    pub account_size_at_open : f32,
    pub account_size_at_close : f32,
}

#[derive(Debug)]
pub struct Trade {
    pub symbol : String,

    pub buy_time : DateTime<Local>,
    pub buy_price : f32,
    pub buy_count : u32,

    pub sell_price : f32,
    pub sell_time : DateTime<Local>,

    // Mainly for visualizing what it's doing.
    pub rounded_position_amount : u32,
    pub rounded_position_unused_amount : u32,

    pub fee_amount : f32,
}

#[derive(Debug)]
pub struct Settings {
    pub account_initial_size : f32,
    pub positions_minimal_amount : f32,
    pub positions_percentage_of_current_account_size : f32,
    pub fee_per_transaction : f32,
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

