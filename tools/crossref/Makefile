#
#  crossref makefile
#
.PHONY: crossref help

help:
	@echo "crossref: mu/core symbol cross reference"

crossref:
	@/opt/mu/bin/mu-sys -l /opt/mu/lib/image.l -q '(image:%require "core")' -l ./crossref.l -q '(crossref "crossref.out")'
	@python3 crossref.py crossref.out

clean:
	@rm -f crossref.out
