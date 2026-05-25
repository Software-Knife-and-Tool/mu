#
#  crossref makefile
#
.PHONY: crossref help

help:
	@echo "crossref: core symbol cross reference"

CORE = ../../../dist/core.sys
FORMAT = ../../../dist/format.sys

crossref:
	@/opt/system-lisp/bin/mu-sys -l $(CORE) -l $(FORMAT) -l ./crossref.l -q '(symbols:crossref "crossref.out")'
	@python3 crossref.py crossref.out

clean:
	@rm -f crossref.out
