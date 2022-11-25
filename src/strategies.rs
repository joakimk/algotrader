pub mod simple_buy_daily_trend_strategy;

use crate::types::*;

pub fn trade_strategy(name : String, chart : &Chart, day: &Day, previous_day: &Day, bar : &Bar, active_trade : &Option<ActiveTrade>) -> Action {
    // Later this might be some other kind of lookup like an enum list, but this works fine for now.
    if name == "simple_buy_daily_trend_strategy" {
        simple_buy_daily_trend_strategy::trade(chart, day, previous_day, bar, active_trade)
    } else {
        Action::None
    }
}
