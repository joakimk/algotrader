# algotrader

WIP: Work in progress. Nothing usable yet.

## Intro

Open source algorithmic trader for developers written in Rust. The idea is that you have to implement strategies in Rust (to enable quick backtests) and also your own HTTP API (in any language) to your specific broker.

I am by no means an expert in any of this, but I have some experience from exploring this as a hobby since about 2020. This algotrader is my forth iteration on the concept and the first one that is open source. Taking what I've learned from the previous iterations into this one.

## Readme driven development

Core concepts and design ideas:

- [ ] Loss is always a possible outcome.
  - Never trade anything you can't loose. Even if everything works correctly, perhaps a technical issue somewhere causes orders not to go through properly, or something happens where the market gaps up or down unexpectedly during the day. This is why you want to risk a fixed amount and not the entire account.
- [ ] Fixed amount positions by default.
  - This will make an account grow slower but safer since it avoids huge losses if you start going exponential in the wrong direction.

- [ ] Backtests
  - [ ] Backtests are essential.
    - Only trade what has been proven to work historically.
  - [ ] Backtests should be nearly instant.
    - You should not feel like skipping backtesting because it's slow, or have to iterate slowly when developing algorithms.
  - [ ] Backtests are two step verification.
    - Only use all available data for final verification to avoid tailoring algorithms to the specific historic data.
  - [ ] Backtests fuzz.
    - By changing initial conditions (like date) it can reveal if a strategy only works when run on a specific day.
- [ ] Day trading
   - The market might be a bit more unreliable on lower timeframes, but at the same time it allows you to work with many different timescales down to seconds.
   - Gives faster feedback on what works and what does not since trades play out over the day instead of days or weeks.
   - Allows for parallel backtesting of multiple days at the same time.
   - Avoids issues with holding positions over multiple days like fees, gap up/down, leverage rebaseing (using say 10x leverage, the underlying market goes down 5% the first day and then up 5% the next, you loose 50% the first day and then gain 50% the next means you now have 75% of the original amount, e.g. `1000 * 0.5 * 1.5 = 750`), ...
   - Using max loss per day by default since some days just don't behave like you expect, better to just wait for the next one.

- [ ] Strategies
  - [ ] No magic values in strategies, everything is configurable.
    - This allows different settings per market and changing settings over time.
    - This also allows for automatically figuring out the best config (possibly by brute force testing).
  - [ ] Many strategies can be active at the same time looking at the same or different markets. There can be many concurrent trades but it's limited to 1 by default.
  - [ ] Ideally it should be able to select strategies and settings based on the recent market conditions, or just what works best in backtests recently.

- [ ] Testing
  - This will be tested (as in automated tests) in a way that allows for a lot of flexibility of implementation. I'm biasing this towards full system tests, e.g. given this config and this data, assert which trades where executed.
  - Unit tests will be used where it makes a lot of sense like for calculations like moving average, ATR, etc.

## TODO

- [ ] Properties to show in result
  - [ ] Maximum drawdown
  - [ ] Exposure time by strategy
    - [ ] Adjusted result given it would be exposed 100% of the time.

# Goal1: Pre-alpha (backtest, two strategies, hardcoded config)

- [x] TradingView import for stock data.
- [ ] Two different stategies (probably one for ranging and one for trending markets).
- [ ] Be able to run on 5 stocks that are as unrelated as possible.
  - [ ] Be able to run one stock without considering days.
  - [ ] Separating into days and running days in parallel.
  - [ ] Run more than one stock at the same time.
  - [ ] Apply strategies and print where it would buy.
  - [ ] Save active position in memory, sell it at end of day if needed.
  - [ ] Collect results and display them.
  - [ ] Manually verify that the trades it took makes sense in reality.
- [ ] Bonus: Full system testing setup (once I have explored what data the system will return, etc.)

## Development

There are very few free data sources that are good so I can't include any data in this repo. I'm developing this using exported CSV data from a paid [TradingView](https://www.tradingview.com/) account. You can use data from any source that provides timestamp/open/close/high/low and volume.

Import data.

```
make import
```

Run backtests and show results.

```
make
```

## Contributions

Contributions are very welcome, but be aware I don't always have a lot of spare time for this project so it could take some time before I can review it. The more well made, consistent with the style of the existing project, and well tested it is the easier it is to include it.

## Broker integration ideas

- The simplest possible API would be one that provides market data during the day and then notifies you to buy or sell when needed. You probably want to run such an integration on the hourly timeframe to not get too many notifications.

## Related resources

- https://www.youtube.com/c/CriticalTrading

## Various backtest inspiration

- Review what is the most essential data to show with results based on The output shown here https://youtu.be/tbronjDYeUs?t=594

## Various algorithm inspiration

- https://youtu.be/4k1_1cLoMKk?t=198
- https://www.youtube.com/watch?v=_9Bmxylp63Y 
- https://www.youtube.com/watch?v=qMXUMjckVhU&ab_channel=TheTransparentTrader
