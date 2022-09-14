all: import
#all: run

run:
	@ # building release version because it runs much faster which is essential for quick backtests
	@cargo build -r
	@target/release/algotrader

import:
	@script/import_tradingview_csv
