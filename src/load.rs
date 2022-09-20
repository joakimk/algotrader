use std::fs;
use regex::Regex;
use chrono::prelude::*;
use std::time::{UNIX_EPOCH, Duration};

use crate::types::*;

pub fn load_chart(path : &str) -> Chart {
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
    let mut day_open_bar = &bars[0];
    let mut previous_bar = &bars[0];
    let mut highest_high_today : f32 = bars[0].high;
    let mut lowest_low_today : f32 = bars[0].low;

    for bar in &bars {
        let bar_date = bar.time.date();

        let last_bar_in_data_set = bar.time == last_bar.time;

        if bar.high > highest_high_today {
            highest_high_today = bar.high;
        }

        if bar.low < lowest_low_today {
            lowest_low_today = bar.low;
        }

        if bar_date > current_date || last_bar_in_data_set {
            let day_close_bar =
                if last_bar_in_data_set {
                    bar
                } else {
                    previous_bar
                };

            days.push(Day {
                date: current_date,
                open_time: day_open_bar.time,
                close_time: day_close_bar.time,
                open: day_open_bar.open,
                high: highest_high_today,
                low: lowest_low_today,
                close: day_close_bar.close,
            });

            day_open_bar = &bar;
            current_date = bar.time.date();
            highest_high_today = bar.high;
            lowest_low_today = bar.low;
        }

        previous_bar = &bar;
    }

    Chart {
        symbol: symbol.to_string(),
        timeframe: timeframe.parse::<i32>().unwrap(),
        bars: bars,
        days: days
    }
}

#[cfg(test)]
mod load_chart {
    use super::*;

    #[test]
    fn test_loading() {
        // I have a support ticket at tradingview asking if I can use some of their data as fixture
        // data and for examples in a public repo. WIP.
        let chart = load_chart("data/15/AZA.json");
        assert_eq!(chart.bars.len(), 10294)
    }

    #[test]
    fn test_days_are_calculated_correctly() {
        let chart = load_chart("data/15/AZA.json");
        let first_bar = &chart.bars[0];
        let last_bar = &chart.bars[chart.bars.len() - 1];
        let last_bar_of_first_day = &chart.bars[33];
        let first_day = &chart.days[0];
        let last_day = &chart.days[chart.days.len() - 1];

        assert_eq!(first_day.date, first_bar.time.date());

        assert_eq!(first_day.open_time, first_bar.time);
        assert_eq!(first_day.open_time, first_bar.time);
        assert_eq!(first_day.close_time, last_bar_of_first_day.time);

        assert_eq!(first_day.open, first_bar.open);
        assert_eq!(first_day.high, 274.3);
        assert_eq!(first_day.low, 266.9);
        assert_eq!(first_day.close, last_bar_of_first_day.close);

        assert_eq!(last_day.close_time, last_bar.time);

        // wip
    }
}
