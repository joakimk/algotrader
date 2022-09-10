# algotrader

## Intro

WIP: Work in progress. Nothing usable yet.

Open source algorithmic trader for developers written in Rust. The idea is that you have to implement strategies in rust and also your own API to your specific broker, thus you probably need to be a developer (or known one) to use this.

I am by no means an expert in any of this, but I have some experience from exploring this as a hobby since about 2020.

## Readme driven development

Things will be checked as they are implemented, until then it's only ideas.

Core concepts:

- [ ] Backtest but not using all available data, then confirm using remaining data.
- [ ] Backtest speed is very important. You should not feel like skipping backtesting because it's slow, or have to iterate slowly when developing algorithms.
- [ ] It always exits all positions before the end of the day. This greatly simplifies things.
- [ ] (configurable) fixed amount positions by default. This avoids huge losses if you start going exponential in the wrong direction.
- [ ] (configurable) max loss per day by default.
- [ ] More things, TODO
