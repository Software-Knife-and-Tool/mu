#
#  tools makefile
#
.PHONY: crossref symbols help

help:
	@echo "symbols: prelude symbol info"

symbols:
	@mu --pipe --load=./symbols.l --quiet-eval='(prelude:symbols \"symbols.out\")'
	@python3 symbols.py ../../dist/prelude/core.l symbols.out

clean:
	@rm -f symbols.out
