use crate::types::*;
use crate::strategy_helpers::*;

pub fn trade(_symbol : &String, _bars : &Vec<Bar>, today: &PartialDay, previous_day: &Day, bar : &Bar, _active_trade : &Option<ActiveTrade>) -> Action {
    if market_closes_in_minutes(today, bar, 30) { return Action::Exit }

    if market_opened_minutes_ago(today, bar, 30) && bar.close < today.open && was_bullish(previous_day) {
        Action::EnterLong
    } else {
        Action::None
    }
}
