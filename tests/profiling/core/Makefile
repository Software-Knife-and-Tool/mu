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
	@manifest build --profile
	@manifest symbols --namespace core --output core.reference
	@manifest profile --config '("core-perf.l", "core-perf")' --output core.profile
	@manifest annotate --prof core.profile --ref core.reference --output core.report
	@cat core.report

clean:
	@rm -rf core.report core.profile core.reference
