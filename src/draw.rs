use cli_candlestick_chart::{Candle, Chart as DrawChart};

use crate::types::*;

#[allow(dead_code)]
pub fn draw_day_results(results : &Vec<DayResult>) {
    let mut candles : Vec<Candle> = Vec::new();


    for r in results {
        let high: f32;
        let low: f32;

        if r.account_size_at_open >= r.account_size_at_close {
            high = r.account_size_at_open;
            low = r.account_size_at_close;
        } else {
            high = r.account_size_at_close;
            low = r.account_size_at_open;
        }

        candles.push(Candle {
           open: r.account_size_at_open.into(),
           high: high.into(),
           low: low.into(),
           close: r.account_size_at_close.into(),
           volume: None,
           timestamp: Some(r.timestamp as i64)
        });
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
