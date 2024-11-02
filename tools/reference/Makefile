#
#  symbol reference makefile
#
.PHONY: mu core help

help:
	@echo "reference: namespace symbol reference"

mu:
	@/opt/mu/bin/mu-sys -l /opt/mu/dist/core.l -l ./reference.l -q '(reference "mu" "reference.out")'
	@python3 reference.py reference.out
	@rm -f reference.out

core:
	@/opt/mu/bin/mu-sys -l /opt/mu/dist/core.l -l ./reference.l -q '(reference "core" "reference.out")'
	@python3 reference.py reference.out
	@rm -f reference.out

mu-verbose:
	@/opt/mu/bin/mu-sys -l /opt/mu/dist/core.l -l ./reference.l -q '(reference "mu" "reference.out")'
	@python3 verbose.py reference.out
	@rm -f reference.out

core-verbose:
	@/opt/mu/bin/mu-sys -l /opt/mu/dist/core.l -l ./reference.l -q '(reference "core" "reference.out")'
	@python3 verbose.py reference.out
	@rm -f reference.out

clean:
	@rm -f reference.out
