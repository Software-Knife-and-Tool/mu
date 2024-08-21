#
#  tools makefile
#
.PHONY: symbols help clean

help:
	@echo "symbols: prelude symbol info"

symbols:
	@mu --pipe --load=./symbols.l --quiet-eval='(symbols \"symbols.out\")'
	@python3 symbols.py ../../dist/core.l symbols.out

clean:
	@rm -f symbols.out
