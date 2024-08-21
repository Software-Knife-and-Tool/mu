#
#  crossref makefile
#
.PHONY: crossref help

help:
	@echo "crossref: mu/core symbol cross reference"

crossref:
	@mu --pipe --load=./crossref.l --quiet-eval='(crossref \"crossref.out\")'
	@python3 crossref.py crossref.out

clean:
	@rm -f crossref.out
