use crate::types::*;
use crate::strategies::*;
use crate::load::*;
use chrono::prelude::*;

// NOTE: Don't pass anything mutable into this to retain the ability to parallelize this if needed later to keep backtesting fast.
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
                fee_amount: settings.fee_per_transaction * 2.0,
            });
        }
    });

    let starting_account_amount = 100f32;
    let mut account_amount = starting_account_amount;
    let mut fee_amount = 0f32;

    for trade in trades.iter() {
        let diff = trade.sell_price / trade.buy_price;
        let position_amount = (trade.buy_count as f32) * trade.buy_price;
        let unused_amount = account_amount - position_amount;
        account_amount = unused_amount + position_amount * diff;
        fee_amount += trade.fee_amount;
    }

    let percent = ((account_amount / starting_account_amount) - 1.0) * 10.0;

    DayResult {
        timestamp: day.open_time.timestamp() as u64,
        time: day.open_time,
        percent: percent,
        fee_amount: fee_amount,
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

#[cfg(test)]
mod simulate_day {
    use super::*;

    #[test]
    fn test_returns_the_expected_result() {
        let settings = Settings {
            account_size: 3000.0,
            position_size: 1000.0,
            fee_per_transaction: 1.0,
        };

        let chart = load_about_a_month_of_stock_data();
        let day = &chart.days[3];

        let day_result = simulate_day(&settings, &chart, &day);

        dbg!(&day_result);

        assert_eq!(day_result.time, Local.ymd(2022, 8, 17).and_hms(9, 0, 0));
        assert_eq!(day_result.fee_amount, 2.0);

        // The diff between buy and sell is 3.35%, but we do not use
        // the entire amount so in total the account change is 3.22%.
        // ruby: (1 - (((198.5 * 5) * (192.05/198.5) + (1000 - (198.5 * 5))) / 1000.0)).round(5) * 100
        assert_eq!(day_result.percent, -(3.225));
    }

    fn load_about_a_month_of_stock_data() -> Chart {
        load_chart("AZA", 15, "data/test/OMXSTO-AZA-2022-08-11-2022-09-15.json")
    }
}
