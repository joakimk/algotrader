# algotrader

WIP: Work in progress. Nothing usable yet.

## Intro

Open source algorithmic trader for developers written in Rust. The idea is that you have to implement strategies in Rust (to enable quick backtests) and also your own HTTP API (in any language) to your specific broker. Thus you probably need to be a developer (or known one) to use this.

I am by no means an expert in any of this, but I have some experience from exploring this as a hobby since about 2020.

## Readme driven development

Core concepts:

- [ ] Loss is always a possible outcome.
  - Never trade anything you can't loose. Even if everything works correctly, perhaps a technical issue somewhere causes orders not to go through properly, or something happens where the market gaps up or down unexpectedly during the day. This is why you want to risk a fixed amount and not the entire account.
- [ ] Backtests are essential.
  - Only trade what has been proven to work historically.
- [ ] Backtests should be nearly instant.
  - You should not feel like skipping backtesting because it's slow, or have to iterate slowly when developing algorithms.
- [ ] Backtests are two step verification.
  - Only use all available data for final verification to avoid tailoring algorithms to the specific historic data.
- [ ] It always exits all positions before the end of the day.
  - This greatly simplifies many things by avoiding fees, gap up/down, leverage rebaseing, ...
- [ ] Fixed amount positions by default.
  - This will make an account grow slower but safer since it avoids huge losses if you start going exponential in the wrong direction.
- [ ] Max loss per day by default.
  - Some days just don't behave like you expect, better to just wait for the next one.
- [ ] No magic values in strategies, everything is configurable.
  - This allows different settings per market and changing settings over time.
  - This also allows for automatically figuring out the best config (possibly by brute force testing).
- [ ] More things, TODO

## Development

There are very few free data sources that are good so I can't include any data in this repo. I'm developing this using exported CSV data from a paid TradingView account.
