use crate::types::*;

pub fn trade(_chart : &Chart, _day: &Day, _previous_day: &Day, _bar : &Bar, active_trade : &Option<ActiveTrade>) -> Action {
    if let None = active_trade {
        Action::EnterLong
    } else {
        Action::Hold
    }
}
