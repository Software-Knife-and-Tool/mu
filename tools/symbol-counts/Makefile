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

clean:
	@rm -f symbols.out
