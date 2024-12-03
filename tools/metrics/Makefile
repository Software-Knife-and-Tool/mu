#
#  symbols makefile
#
.PHONY: mu core help clean

help:
	@echo "symbols: core symbol counts"

core:
	@/opt/mu/bin/mu-sys -l /opt/mu/dist/core.l -l ./symbols.l -q '(symbols "core" "symbols.out")'
	@python3 symbols.py ../../dist/core.l symbols.out
	@rm -f symbols.out

prelude:
	@/opt/mu/bin/mu-sys -l /opt/mu/dist/core.l -l /opt/mu/dist/prelude.l ./symbols.l -q '(symbols "prelude" "symbols.out")'
	@python3 symbols.py ../../dist/prelude.l symbols.out
	@rm -f symbols.out

clean:
	@rm -f symbols.out
