#
# core library profiling
#
.PHONY: help base clean commit current verbose summary metrics

TMPF:=$(shell mktemp)
BIN = ./scripts

help:
	@echo "core library profiling makefile -----------------"
	@echo
	@echo "--- options"
	@echo "    report - produce annotated report"

report:
	@mux build --profile
	@mux symbols --namespace core --output core.reference
	@mux profile --config '("core-perf.l", "core-perf")' --output core.profile
	@mux annotate --prof core.profile --ref core.reference --output core.report
	@cat core.report

clean:
	@rm -rf core.report core.profile core.reference
