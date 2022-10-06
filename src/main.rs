mod load;
mod draw;
mod types;
mod simulator;
mod strategies;

use rayon::prelude::*;
use crate::types::*;

fn main() {
    let chart = load::load_chart("AZA", 15, "data/local/15/AZA.json");

    let results : Vec<DayResult> = chart.days.par_iter()
        .map(|day| { dbg!("map"); simulator::simulate_day(&chart, day) })
        .collect();

    dbg!(chart.days.len());
    dbg!(results.len());
    let mut fixed_change = 100f32;
    // exponential_change
    for r in results {
        // this is just a placeholder calculation
        fixed_change = fixed_change + r.percent;
    }
    dbg!(fixed_change);

    // next:
    // x cleanup chart loading code, put that into a separate file
    // x separate into days and run them in parallel
    // - figure out how simulated trading would work
    // - run more than one stock at the same time

    //dbg!(&chart.bars[0]);
    //dbg!(&chart.bars[10293]);
    dbg!(&chart.days[0]);
    dbg!(&chart.days[304]);

    //draw::draw_chart(chart)
}
