use crate::types::*;

// The idea with this strategy is to look for days that significantly gapped up when the market opens compared to where it closed the day before.
//
// Then wait for it to retrace back to the general area of yesterdays close and go long (buy) risking very little (just below yesterdays close).
//
// If possible let it hold that position until just before day close. This strategy could then be used for manual entries for when you don't have a API integration with your broker.

pub fn trade(_bar : &Bar) {
    //dbg!(_bar);
}
