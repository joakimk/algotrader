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
pub struct Day {
    pub date : Date<Local>,
    //pub open_time : DateTime<Local>,
    //pub close_time : DateTime<Local>,

    //pub open : f32,
    //pub high : f32,
    //pub low : f32,
    //pub close : f32,
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

fn main() {
    let chart = load_chart("data/15/AZA.json");

    // next:
    // - separate into days and run them in parallel
    // - figure out how simulated trading would work
    // - run more than one stock at the same time

    dbg!(&chart.days[305]);
}

fn load_chart(path : &str) -> Chart {
    let re = Regex::new(r"data/(.+)/(.+)\.json").unwrap();
    let cap = re.captures(path).unwrap();
    let timeframe = &cap[1];
    let symbol = &cap[2];

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

    let mut current_date = bars[0].time.date();
    let mut days : Vec<Day> = Vec::new();

    // TODO: This does not include a day for the last day since there
    //       will never be a bar day newer than the current date.
    //
    //       I think I want it to add a day once it reaches the end
    //       of the current day in order to be able to collect
    //       open/close, etc.
    //
    //       But maybe just a group would work better?
    for bar in &bars {
        let bar_date = bar.time.date();

        if bar_date > current_date {
            days.push(Day {
                date: current_date
            });

            current_date = bar.time.date();
        }
    }

    Chart {
        symbol: symbol.to_string(),
        timeframe: timeframe.parse::<i32>().unwrap(),
        bars: bars,
        days: days
    }
}
