use crate::types::*;

pub fn trade(bar : &Bar, active_trade : &mut Option<ActiveTrade>) -> TradeAction {
    if let None = active_trade {
        TradeAction::Long
    } else {
        TradeAction::None
    }
}
