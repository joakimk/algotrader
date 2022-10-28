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
        position_size: 1000.0
    };

    let results : Vec<DayResult> = chart.days.par_iter()
        .map(|day| { simulator::simulate_day(&settings, &chart, day) })
        .collect();

    draw::draw_day_results(&results);

    dbg!(chart.days.len());
    dbg!(&chart.days[0].date);
    dbg!(&chart.days[304].date);
    dbg!(&results[0]);

    //draw::draw_chart(chart)
}
