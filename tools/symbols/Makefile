#
#  tools makefile
#
.PHONY: crossref symbols help

help:
	@echo "crossref: mu symbol cross reference"
	@echo "symbols: mu symbol info"

crossref:
	@mu --pipe --load=./crossref.l --quiet-eval='(prelude:crossref \"crossref.out\")'
	@python3 crossref.py crossref.out

symbols:
	@mu --pipe --load=./symbols.l --quiet-eval='(prelude:symbols \"symbols.out\")'
	@python3 symbols.py ../dist/prelude.l symbols.out

clean:
	@rm -f crossref.out symbols.out
