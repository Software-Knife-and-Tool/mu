#
#  symbols makefile
#
.PHONY: mu core help clean

help:
	@echo "symbols: core symbol counts"

core:
	@/opt/mu/bin/mu-sys -l ../../dist/core.sys -l ./symbols.l -q '(symbols:metrics "core" "symbols.out")'
	@python3 symbols.py ../../dist/core.sys symbols.out
	@rm -f symbols.out

prelude:
	@/opt/mu/bin/mu-sys -l ../../dist/core.sys -q '(core:require "prelude")' -l ./symbols.l -q '(symbols:metrics "prelude" "symbols.out")'
	@python3 symbols.py ../../dist/prelude.l symbols.out
	@rm -f symbols.out

clean:
	@rm -f symbols.out
