use crate::types::*;
use crate::strategy_helpers::*;

pub fn trade(_chart : &Chart, today: &PartialDay, previous_day: &Day, bar : &Bar, _active_trade : &Option<ActiveTrade>) -> Action {
    if market_opened_minutes_ago(today, bar, 30) && was_bullish(previous_day) {
        Action::EnterLong
    } else if market_closes_in_minutes(today, bar, 30) {
        Action::Exit
    } else {
        Action::None
    }
}
