pub mod simple_buy_daily_trend_strategy;
pub mod gap_up_reversal_long_strategy;

use crate::types::*;

pub fn trade_strategy(name : String, symbol : &String, bars : &Vec<Bar>, today: &PartialDay, previous_day: &Day, bar : &Bar, active_trade : &Option<ActiveTrade>) -> Action {
    // Later this might be some other kind of lookup like an enum list, but this works fine for now.
    if name == "simple_buy_daily_trend_strategy" {
        simple_buy_daily_trend_strategy::trade(symbol, bars, today, previous_day, bar, active_trade)
    } else if name == "gap_up_reversal_long_strategy" {
        gap_up_reversal_long_strategy::trade(symbol, bars, today, previous_day, bar, active_trade)
    }
    else { Action::None }
}
