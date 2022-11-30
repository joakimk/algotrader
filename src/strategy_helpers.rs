use crate::types::*;
use chrono::{Duration};
use chrono::prelude::*;

pub fn was_bullish(day : &Day) -> bool { day.close > day.open }

// Based on experience you should never have time expecations in the code since markets can close
// at different times on different days (there is usually a list of exception days at the broker).
pub fn market_opened_minutes_ago(today : &PartialDay, bar : &Bar, minutes : i64) -> bool { bar.time == add_min(today.open_time, minutes) }
pub fn market_closes_in_minutes(today : &PartialDay, bar : &Bar, minutes : i64) -> bool { bar.time >= sub_min(today.close_time, minutes) }

pub fn add_min(time : DateTime<Local>, minutes : i64) -> DateTime<Local> { time + Duration::minutes(minutes) }
pub fn sub_min(time : DateTime<Local>, minutes : i64) -> DateTime<Local> { time - Duration::minutes(minutes) }

#[allow(dead_code)]
pub fn close_above_moving_average(bars : &Vec<Bar>, bar : &Bar, length : usize) -> bool {
    if bars.len() > length {
        bar.close > moving_average(bars, length)
    } else {
        false
    }
}

#[allow(dead_code)]
pub fn close_below_moving_average(bars : &Vec<Bar>, bar : &Bar, length : usize) -> bool {
    if bars.len() > length {
        bar.close < moving_average(bars, length)
    } else {
        false
    }
}

#[allow(dead_code)]
pub fn moving_average(bars : &Vec<Bar>, length : usize) -> f32{
    let total_length = bars.len();
    let bars_within_moving_average_period = &bars[(total_length - length - 1)..(total_length - 1)];

    let mut sum = 0.0;
    for bar in bars_within_moving_average_period {
        sum += bar.close;
    }

    sum / (length as f32)
}
