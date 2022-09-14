use std::fs;
use serde::Deserialize;
use regex::Regex;

#[derive(Debug, Deserialize)]
pub struct Bar {
    pub timestamp: i64,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}

#[derive(Debug)]
pub struct Chart {
    pub symbol : String,
    pub timeframe : i32,
    pub bars : Vec<Bar>,
}

fn main() {
    let chart = load_chart("data/1/CAST.json");

    dbg!(chart);
}

fn load_chart(path : &str) -> Chart {
    let data = fs::read_to_string(path)
        .expect("Unable to read file at {path}.");

    let bars: Vec<Bar> = serde_json::from_str(&data)
        .expect("JSON in {path} does not have correct format.");

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
