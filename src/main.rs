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

    dbg!(&chart.bars[0]);
    dbg!(&chart.bars[10293]);
    dbg!(&chart.days[0]);
    dbg!(&chart.days[304]);
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

    let mut all_bars : Vec<Bar> = Vec::new();
    for raw_bar in raw_bars {
        let duration = UNIX_EPOCH + Duration::from_secs(raw_bar.timestamp);
        let time = DateTime::<Local>::from(duration);

        all_bars.push(Bar {
            time: time,
            timestamp: raw_bar.timestamp,
            open: raw_bar.open,
            high: raw_bar.high,
            low: raw_bar.low,
            close: raw_bar.close,
            volume: raw_bar.volume,
        })
    };

    // Start bars at the start of a day and end it at the
    // end of a day so we can assume we only have complete
    // days when running backtests.
    let initial_date : Date<Local> = all_bars[0].time.date();
    let last_date : Date<Local> = all_bars[all_bars.len() - 1].time.date();
    let mut bars : Vec<Bar> = Vec::new();
    for bar in all_bars {
        let bar_date = bar.time.date();
        if bar_date != initial_date && bar_date != last_date {
            bars.push(bar)
        }
    }

    // Build days
    let mut current_date = bars[0].time.date();
    let mut days : Vec<Day> = Vec::new();
    let last_bar = &bars[bars.len() - 1];

    for bar in &bars {
        let bar_date = bar.time.date();

        if bar_date > current_date || bar.time == last_bar.time {
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
