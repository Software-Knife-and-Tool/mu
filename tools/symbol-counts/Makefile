#
#  symbols makefile
#
.PHONY: mu core help clean

help:
	@echo "symbols: core symbol counts"

core:
	@mu --pipe --load=./symbols.l --quiet-eval='(symbols \"core\" \"symbols.out\")'
	@python3 symbols.py ../../dist/core.l symbols.out
	@rm -f symbols.out

clean:
	@rm -f symbols.out
