use crate::types::*;
use crate::strategies::*;

pub fn simulate_day(settings: &Settings, chart: &Chart, day: &Day) -> DayResult {
    let mut trades = Vec::new();

    bars_today(chart, day).iter().for_each( |bar| {
        simple_buy_trend_strategy::trade(bar);

        // WIP: Overslimplified: Buy open, sell close of day.
        if trades.len() == 0 {
            let buy_time = day.open_time;
            let buy_price = day.open;
            let buy_count = (settings.position_size / buy_price) as u32;
            let buy_total = (buy_count as f32) * buy_price;

            let sell_time = day.close_time;
            let sell_price = day.close;

            trades.push(Trade {
                symbol: chart.symbol.to_string(),
                buy_time: buy_time,
                sell_time: sell_time,
                buy_price: buy_price,
                buy_count: buy_count,
                rounded_position_amount: ((buy_count as f32) * buy_price) as u32,
                rounded_position_unused_amount: (settings.position_size - buy_total) as u32,
                sell_price: sell_price,
            });
        }
    });

    let starting_account_amount = 100f32;
    let mut account_amount = starting_account_amount;

    for trade in trades.iter() {
        let diff = trade.sell_price / trade.buy_price;
        let position_amount = (trade.buy_count as f32) * trade.buy_price;
        let unused_amount = account_amount - position_amount;
        account_amount = unused_amount + position_amount * diff;
    }

    let close_percent = ((account_amount / starting_account_amount) - 1.0) * 100.0;

    DayResult {
        timestamp: day.open_time.timestamp() as u64,
        close_percent: close_percent,
        low_percent: close_percent,
        high_percent: close_percent,
        trades: trades,
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
