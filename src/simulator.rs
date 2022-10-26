use crate::types::*;
use crate::strategies::*;

pub fn simulate_day(chart: &Chart, day: &Day) -> DayResult {
    bars_today(chart, day).iter().for_each( |bar| {
        simple_buy_trend_strategy::trade(bar);
    });

    DayResult {
        timestamp: day.open_time.timestamp() as u64,
        close_percent: 1f32,
        low_percent: -0.5f32,
        high_percent: 1.2f32
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
