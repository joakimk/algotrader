#all: import
all: run
#all: test

.PHONY: run test import

run:
	@ # building release version because it runs much faster which is essential for quick backtests
	@cargo build -r
	@target/release/algotrader

test:
	@cargo test

import:
	@script/import_tradingview_csv
