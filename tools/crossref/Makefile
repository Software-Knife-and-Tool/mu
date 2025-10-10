#
#  crossref makefile
#
.PHONY: crossref help

help:
	@echo "crossref: core symbol cross reference"

crossref:
	@/opt/mu/bin/mu-sys -l ../../dist/core.sys -l ./crossref.l -q '(symbols:crossref "crossref.out")'
	@python3 crossref.py crossref.out

clean:
	@rm -f crossref.out
