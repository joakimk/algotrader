use crate::types::*;
use crate::strategies::*;

pub fn simulate_day(settings: &Settings, chart: &Chart, day: &Day, previous_day: &Day, account_size_at_open: f32) -> DayResult {
    let mut trades = Vec::new();
    let mut active_trade : Option<ActiveTrade> = None;
    let mut account_amount = account_size_at_open;
    let mut fee_amount = 0f32;

    let max_position_size = settings.position_minimal_amount.max((settings.position_percentage_of_current_account_size / 100.0) * account_size_at_open);

    bars_today(chart, day).iter().for_each( |bar| {
        for strategy in &settings.enabled_strategies {
            let action = trade_strategy(strategy.into(), chart, day, previous_day, bar, &active_trade);

            // WIP
            match action {
                Action::None => {}
                Action::EnterLong => {
                    // We ignore enter signals if we're already in a trade to keep strategy code simple
                    if let None = active_trade {
                        let buy_time = bar.time;
                        let buy_price = bar.close;

                        let buy_count = (max_position_size / buy_price) as u32;

                        active_trade = Some(ActiveTrade {
                            symbol: chart.symbol.to_string(),
                            buy_time: buy_time,
                            buy_price: buy_price,
                            buy_count: buy_count,
                            fee_amount: settings.fee_per_transaction * 2.0,
                        });

                        //dbg!(&active_trade);
                    }
                }
                //Action::EnterShort => { panic!("Not implemented yet.") }
                Action::Exit => {
                    if let None = active_trade {
                        // Nothing to do.
                    } else {
                        let at = active_trade.as_ref().unwrap();
                        let buy_count = at.buy_count;
                        let buy_price = at.buy_price;
                        let buy_total = (buy_count as f32) * buy_price;

                        let trade = Trade {
                            symbol: at.symbol.clone().into(),
                            buy_time: at.buy_time,
                            sell_time: bar.time,
                            buy_price: buy_price,
                            buy_count: buy_count,
                            rounded_position_amount: ((buy_count as f32) * buy_price) as u32,
                            rounded_position_unused_amount: (max_position_size - buy_total) as u32,
                            sell_price: bar.close,
                            fee_amount: settings.fee_per_transaction * 2.0,
                        };

                        let diff = trade.sell_price / trade.buy_price;
                        let position_amount = (trade.buy_count as f32) * trade.buy_price;
                        let unused_amount = account_amount - position_amount;
                        account_amount = unused_amount + position_amount * diff;
                        fee_amount += trade.fee_amount;

                        if account_amount < settings.position_minimal_amount {
                            // Handle this more gracefully if it ever becomes an issue, e.g. just stop backtest and show results.
                            panic!("Strategy is performing very poorly. Account amount (account_amount) is lower than the minimal position amount (position_minimal_amount).")
                        }

                        trades.push(trade);

                        active_trade = None;
                    }
                }
            }
        }
    });

    if let None = active_trade {
        let account_size_at_close = account_amount;

        DayResult {
            timestamp: day.open_time.timestamp() as u64,
            time: day.open_time,
            fee_amount: fee_amount,
            trades: trades,
            account_size_at_open: account_size_at_open,
            account_size_at_close: account_size_at_close,
        }
    } else {
        panic!("There is still an active trade around at day close ({:?}): {:?}. Make sure strategies sell positions before close.", day.close_time, active_trade.unwrap())
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
    use crate::load::*;
    use chrono::prelude::*;

    #[test]
    fn test_returns_the_expected_result() {
        let settings = Settings {
            account_initial_size: 3000.0,
            position_minimal_amount: 1300.0,
            position_percentage_of_current_account_size: 45.0,
            fee_per_transaction: 1.0,
            enabled_strategies: [ "simple_buy_daily_trend_strategy".into() ].to_vec(),
        };

        let chart = load_about_a_month_of_stock_data();
        let day = &chart.days[3];

        let account_size_before_day = settings.account_initial_size;
        let day_result = simulate_day(&settings, &chart, &day, account_size_before_day);

        //dbg!(&day_result);

        assert_eq!(day_result.time, Local.ymd(2022, 8, 17).and_hms(9, 0, 0));
        assert_eq!(day_result.fee_amount, 2.0);

        assert_eq!(day_result.account_size_at_open, settings.account_initial_size);

        // We use 198.5 * 6 for the position, that is 1191
        // 1300 + 1191 = 109 which is unused.
        // 1191 decreases by 3.25% to 1152.3
        // What remains is 1152.3 which if you add the unused 109 you get 1261.3.
        assert_eq!(day_result.account_size_at_close, 2961.3);
    }

    #[test]
    fn test_can_use_larger_positions_as_the_account_grows() {
        let settings = Settings {
            account_initial_size: 3000.0,
            position_minimal_amount: 1300.0,
            position_percentage_of_current_account_size: 45.0,
            fee_per_transaction: 1.0,
            enabled_strategies: [ "simple_buy_daily_trend_strategy".into() ].to_vec(),
        };

        let chart = load_about_a_month_of_stock_data();
        let day = &chart.days[3];

        let account_size_before_day = 3500.0;
        let day_result = simulate_day(&settings, &chart, &day, account_size_before_day);

        //dbg!(&day_result);

        assert_eq!(day_result.trades.len(), 1);
        let trade = &day_result.trades[0];
        let position_size_used : f32 = (trade.rounded_position_amount + trade.rounded_position_unused_amount) as f32;

        // We expect it to be 45% of 3500.0 (which should be 1575 but we use a rounded amount).
        assert_eq!(position_size_used, 1574.0);
    }

    fn load_about_a_month_of_stock_data() -> Chart {
        load_chart("AZA", 15, "data/test/OMXSTO-AZA-2022-08-11-2022-09-15.json")
    }
}
