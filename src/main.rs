mod load;
mod draw;
mod types;
mod simulator;
mod strategies;
mod strategy_helpers;

//use rayon::prelude::*;
use crate::types::*;
use std::env;

fn main() {
    let chart = load::load_chart("AZA", 15, "data/local/15/AZA.json");

    let settings = Settings {
        account_initial_size: 3000.0,

        //account_max_loss_percent_per_day: "todo later",

        position_minimal_amount: 1300.0,
        position_percentage_of_current_account_size: 45.0,

        enabled_strategies: [ "simple_buy_daily_trend_strategy".into() ].to_vec(),
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
    let mut previous_day = &chart.days[0];
    let results : Vec<DayResult> = chart.days.iter()
        .map(|day| {
            let account_size_at_open = current_account_size - previous_day_fees;
            let result = simulator::simulate_day(&settings, &chart, day, previous_day, account_size_at_open);
            previous_day = day;
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
