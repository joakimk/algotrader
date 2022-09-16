use std::fs;
use serde::Deserialize;
use regex::Regex;
use chrono::prelude::*;
use chrono::Local;
use std::time::{UNIX_EPOCH, Duration};

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
pub struct Chart {
    pub symbol : String,
    pub timeframe : i32,
    pub bars : Vec<Bar>,
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

fn main() {
    let chart = load_chart("data/15/AZA.json");

    // next:
    // - separate into days and run them in parallel
    // - figure out how simulated trading would work
    // - run more than one stock at the same time

    dbg!(chart);
}

fn load_chart(path : &str) -> Chart {
    let data = fs::read_to_string(path)
        .expect("Unable to read file at {path}.");

    let raw_bars: Vec<RawBar> = serde_json::from_str(&data)
        .expect("JSON in {path} does not have correct format.");

    let mut bars : Vec<Bar> = Vec::new();
    for raw_bar in raw_bars {
        let duration = UNIX_EPOCH + Duration::from_secs(raw_bar.timestamp);
        let time = DateTime::<Local>::from(duration);

        bars.push(Bar {
            time: time,
            timestamp: raw_bar.timestamp,
            open: raw_bar.open,
            high: raw_bar.high,
            low: raw_bar.low,
            close: raw_bar.close,
            volume: raw_bar.volume,
        })
    };

    let re = Regex::new(r"data/(.+)/(.+)\.json").unwrap();
    let cap = re.captures(path).unwrap();
    let timeframe = &cap[1];
    let symbol = &cap[2];

    Chart {
        symbol: symbol.to_string(),
        timeframe: timeframe.parse::<i32>().unwrap(),
        bars: bars
    }
}
