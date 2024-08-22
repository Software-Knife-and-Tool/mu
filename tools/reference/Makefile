#
#  symbol reference makefile
#
.PHONY: mu core help

help:
	@echo "reference: namespace symbol reference"

mu:
	@mu --pipe --load=./reference.l --quiet-eval='(reference \"mu\" \"reference.out\")'
	@python3 reference.py reference.out
	@rm -f reference.out

core:
	@mu --pipe --load=./reference.l --quiet-eval='(reference \"core\" \"reference.out\")'
	@python3 reference.py reference.out
	@rm -f reference.out

clean:
	@rm -f reference.out
