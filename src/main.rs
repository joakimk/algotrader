mod load;
mod draw;
mod types;
mod simulator;
mod strategies;

use rayon::prelude::*;
use crate::types::*;

fn main() {
    let chart = load::load_chart("AZA", 15, "data/local/15/AZA.json");

    let settings = Settings {
        position_size: 1000.0,
        fee_per_transaction: 0f32, // the broker I intend to use for initial testing has zero fees for stocks
    };

    // Later it should vary settings (and possibly as a setting: start date and end date)
    // in order to fuzz entry conditions and settings to ensure strategies does not
    // accidentally produce good results just in one unique case.
    //let variations : Vec<Settings> = [
    //]
    //
    // Here is where we'll use the parallelism. It could even be run as a distributed calculations
    // over many machines by just splitting up the list.
    //variations.par_iter()

    // If possible have this be a plain map of results so that we can
    // run a par_iter or even a distributed version of that in case you
    // want to backtest something that is resource intensive even without
    // running a lot of different settings (something like backtesting every
    // tick in the last 10 years over multiple instruments and strategies).
    let results : Vec<DayResult> = chart.days.iter()
        .map(|day| { simulator::simulate_day(&settings, &chart, day) })
        .collect();

    draw::draw_day_results(&results);

    //dbg!(chart.days.len());
    //dbg!(&chart.days[0].date);
    //dbg!(&chart.days[304].date);
    //dbg!(&results[0]);

    //draw::draw_chart(chart)
}
