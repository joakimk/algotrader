mod load;
mod draw;
mod types;
mod simulator;
mod strategies;

//use rayon::prelude::*;
use crate::types::*;

fn main() {
    let chart = load::load_chart("AZA", 15, "data/local/15/AZA.json");

    let settings = Settings {
        account_initial_size: 3000.0,

        //no_new_trades_if_lost_more_than_percent_per_day: "todo",
        //no_new_trades_if_account_size_is_below: 2000.0,
        position_minimal_amount: 1300.0,
        position_percentage_of_current_account_size: 45.0,

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

    let mut current_account_size = settings.account_initial_size;
    let mut previous_day_fees = 0f32;
    let results : Vec<DayResult> = chart.days.iter()
        .map(|day| {
            let account_size_at_open = current_account_size - previous_day_fees;
            let result = simulator::simulate_day(&settings, &chart, day, account_size_at_open);
            previous_day_fees = result.fee_amount;
            current_account_size = result.account_size_at_close;
            result
        })
        .collect();

    draw::draw_day_results(&results);

    //dbg!(chart.days.len());
    //dbg!(&chart.days[0].date);
    //dbg!(&chart.days[304].date);
    //dbg!(&results[0]);

    //draw::draw_chart(chart)
}
