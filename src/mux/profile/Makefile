#
#  profiling makefile
#
.PHONY: profile help

help:
	@echo "profile: function call profiler"

profile:
	@mu	\
		--pipe				\
		--load=../../dist/core.l	\
		--load=../../dist/prelude.l	\
		--load=./profile.l		\
		--quiet-eval='(%profile%:profile \"prof-src.l\" \"profile.out\")'

	@python3 profile.py profile.out

clean:
	@rm -f profile.out
