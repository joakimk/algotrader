# algotrader

## Intro

WIP: Work in progress. Nothing usable yet.

Open source algorithmic trader for developers written in Rust. The idea is that you have to implement strategies in rust and also your own API to your specific broker, thus you probably need to be a developer (or known one) to use this.

I am by no means an expert in any of this, but I have some experience from exploring this as a hobby since about 2020.

## Readme driven development

Things will be checked as they are implemented, until then it's only ideas.

Core concepts:

- [ ] Backtests are core to this.
  - Do not trade anything that has not been proven to work historically.
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
- [ ] More things, TODO
