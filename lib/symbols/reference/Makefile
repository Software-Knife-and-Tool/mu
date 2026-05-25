#
#  symbol reference makefile
#
.PHONY: mu core help

help:
	@echo "reference: namespace symbol reference"

CORE = ../../../dist/core.sys
FORMAT = ../../../dist/format.sys

mu:
	@/opt/system-lisp/bin/mu-sys -l $(CORE) -l $(FORMAT) -l ./reference.l -q '(symbols:reference "mu" "reference.out")'
	@python3 reference.py reference.out
	@rm -f reference.out

core:
	@/opt/system-lisp/bin/mu-sys -l $(CORE) -l $(FORMAT) -l ./reference.l -q '(symbols:reference "core" "reference.out")'
	@python3 reference.py reference.out
	@rm -f reference.out

prelude:
	@/opt/system-lisp/bin/mu-sys -l $(CORE) -l $(FORMAT) -q '(core:require "prelude")' -l ./reference.l -q '(symbols:reference "prelude" "reference.out")'
	@python3 reference.py reference.out
	@rm -f reference.out

mu-verbose:
	@/opt/system-lisp/bin/mu-sys -l $(CORE) -l $(FORMAT) -l ./reference.l -q '(symbols:reference "mu" "reference.out")'
	@python3 verbose.py reference.out
	@rm -f reference.out

core-verbose:
	@/opt/system-lisp/bin/mu-sys -l $(CORE) -l $(FORMAT) -l ./reference.l -q '(symbols:reference "core" "reference.out")'
	@python3 verbose.py reference.out
	@rm -f reference.out

prelude-verbose:
	@/opt/system-lisp/bin/mu-sys -l $(CORE) -l $(FORMAT) -q '(core:require "prelude")' -l ./reference.l -q '(symbols:reference "prelude" "reference.out")'
	@python3 verbose.py reference.out
	@rm -f reference.out

clean:
	@rm -f reference.out
