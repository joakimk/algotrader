use crate::types::*;
use crate::strategies::*;

pub fn simulate_day(chart: &Chart, day: &Day) -> DayResult {
    bars_today(chart, day).iter().for_each( |bar| {
        simple_buy_trend_strategy::trade(bar);
    });

    DayResult {
        percent: 10f32,
    }
}

// Optimization: We copy here, if that turns out to be slow, change that.
fn bars_today(chart: &Chart, day: &Day) -> Vec<Bar> {
    let mut bars : Vec<Bar> = Vec::new();

    for bar in &chart.bars {
        if bar.time >= day.open_time && bar.time <= day.close_time {
            bars.push(*bar);
        }
    }

    bars
}
