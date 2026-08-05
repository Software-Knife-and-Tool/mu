#
#  symbols makefile
#
.PHONY: mu core help clean

help:
	@echo "symbols: core symbol counts"

CORE = ../../../dist/core.sys
FORMAT = ../../../dist/format.sys

core:
	@/opt/mu/bin/mu-sys -l $(CORE) -l $(FORMAT) -l ./symbols.l -q '(symbols:metrics "core" "symbols.out")'
	@python3 symbols.py $(CORE) symbols.out
	@rm -f symbols.out

prelude:
	@/opt/mu/bin/mu-sys -l $(CORE) -l $(FORMAT) -q '(core:require "prelude")' -l ./symbols.l -q '(symbols:metrics "prelude" "symbols.out")'
	@python3 symbols.py ../../dist/prelude.l symbols.out
	@rm -f symbols.out

clean:
	@rm -f symbols.out
