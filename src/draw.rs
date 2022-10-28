use cli_candlestick_chart::{Candle, Chart as DrawChart};

use crate::types::*;

#[allow(dead_code)]
pub fn draw_day_results(results : &Vec<DayResult>) {
    let todo_this_duplicates_settings = 1;
    let minimal_position_amount = 10f32;
    let position_amount = minimal_position_amount;
    let account_starting_amount = 1000f32;
    let mut account_amount = account_starting_amount;

    let mut candles : Vec<Candle> = Vec::new();

    for r in results {
        let close_account_amount = account_amount + (position_amount * (1.0 + (r.close_percent / 100.0))) - position_amount;
        assert!(account_amount >= minimal_position_amount);

        let high_account_amount = account_amount + (position_amount * (1.0 + (r.high_percent / 100.0))) - position_amount;
        let low_account_amount = account_amount + (position_amount * (1.0 + (r.low_percent / 100.0))) - position_amount;

        // candle

        candles.push(Candle {
           open: account_amount.into(),
           high: high_account_amount.into(),
           low: low_account_amount.into(),
           close: close_account_amount.into(),
           volume: None,
           timestamp: Some(r.timestamp as i64)
        });

        account_amount = close_account_amount;
    }

    // If the time period is long, combine into weeks or months?

    // Create and display the chart
    let mut draw_chart = DrawChart::new(&candles);

    // Set the chart title
    draw_chart.set_name("Results by day".to_string());

    draw_chart.draw();
}

#[allow(dead_code)]
pub fn draw_chart(chart : Chart) {
    let mut candles : Vec<Candle> = Vec::new();

    // This could also draw entry and exit as special bars?

    //let mut current_date = chart.bars[0].time.date();
	for bar in chart.bars {
        // And day changes. This implementation is kind of a hack.
        // Maybe it would be easy to add support to the library for this.
        //let bar_date = bar.time.date();
        //if bar_date != current_date {
        //    current_date = bar_date;

        //    candles.push(Candle {
        //        open: (bar.open * 1.05) as f64,
        //        close: (bar.close * 0.95) as f64,
        //        high: (bar.open * 1.05) as f64,
        //        low: (bar.close * 0.95) as f64,
        //        volume: Some(0.0),
        //        timestamp: Some(bar.timestamp as i64)
        //    })
        //}

        candles.push(Candle {
           open: bar.open.into(),
           high: bar.high.into(),
           low: bar.low.into(),
           close: bar.close.into(),
           volume: Some(bar.volume as f64),
           timestamp: Some(bar.timestamp as i64)
        })
    }

    // Create and display the chart
    let mut draw_chart = DrawChart::new(&candles);

    // Set the chart title
    draw_chart.set_name(chart.symbol);

    draw_chart.draw();
}
