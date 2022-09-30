mod load;
mod draw;
mod types;
mod simulator;
mod strategies;

use rayon::prelude::*;
use crate::types::*;

fn main() {
    let chart = load::load_chart("AZA", 15, "data/local/15/AZA.json");

    let mut results : Vec<DayResult> = Vec::new();

    chart.days.par_iter().map(|day| {
        simulator::simulate_day(&chart, day);
    });

    // wip get data back
    //.reduce(|| (DayResult { percent: 0 }), |a, b| {
    //})

    //dbg!(results);

    // next:
    // - cleanup chart loading code, put that into a separate file
    // - separate into days and run them in parallel
    // - figure out how simulated trading would work
    // - run more than one stock at the same time

    //dbg!(&chart.bars[0]);
    //dbg!(&chart.bars[10293]);
    dbg!(&chart.days[0]);
    dbg!(&chart.days[304]);

    //draw::draw_chart(chart)
}
