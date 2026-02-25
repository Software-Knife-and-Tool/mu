#
# core library profiling
#
.PHONY: help base clean commit current verbose summary metrics

TMPF:=$(shell mktemp)
BIN = ./scripts
MU_SYS = ../../../target/release/mu-sys

help:
	@echo "core library profiling makefile -----------------"
	@echo
	@echo "--- options"
	@echo "    null - dry run"
	@echo "    core - record core benchmark"
	@echo "    report - produce annotated report"

null:
	$(MU_SYS) -l../../../dist/core.sys -l core.l -q '(core-perf)'

core:
	@perf record -g -e cpu_core/cycles/P $(MU_SYS) -l../../../dist/core.sys -l perf.l -q '(core-perf)'

report: core
	@perf stat --repeat 100 $(MU_SYS) -l../../../dist/core.sys -l perf.l -q '(core-perf)'
	@perf report -g --stdio > perf.report

clean:
	@rm -rf core.report core.profile core.reference
