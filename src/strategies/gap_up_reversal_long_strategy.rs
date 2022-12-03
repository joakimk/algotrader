use crate::types::*;
use crate::strategy_helpers::*;

// The idea with this strategy is to look for days that significantly gapped up when the market opens (or within 15-30 minutes of market open) compared to where it closed the day before.
//
// Then wait for it to retrace back to the general area of yesterdays close and go long (buy) risking very little (just below yesterdays close).
//
// If possible let it hold that position until just before day close. This strategy could then be used for manual entries for when you don't have a API integration with your broker.

pub fn trade(_symbol : &String, _bars : &Vec<Bar>, today: &PartialDay, previous_day: &Day, bar : &Bar, _active_trade : &Option<ActiveTrade>) -> Action {
    if market_closes_in_minutes(today, bar, 30) { return Action::Exit }

    // todo pullback
    // todo perhaps not gap only but also trade up early in day?
    if today.open > previous_day.close * 1.008 {
        Action::EnterLong
    } else {
        Action::None
    }
}
