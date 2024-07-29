#
#  tools makefile
#
.PHONY: crossref help

help:
	@echo "crossref: mu/prelude symbol cross reference"

crossref:
	@mu --pipe --load=./crossref.l --quiet-eval='(prelude:crossref \"crossref.out\")'
	@python3 crossref.py crossref.out

clean:
	@rm -f crossref.out
