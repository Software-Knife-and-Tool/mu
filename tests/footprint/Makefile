#
# footprint makefile
#
.PHONY: help base clean commit current verbose summary metrics

NTESTS ?= 20
TMPF:=$(shell mktemp)
BIN = ./scripts

help:
	@echo "mu footprint makefile -----------------"
	@echo
	@echo "--- footprint options"
	@echo "    base - establish baseline report"
	@echo "    current - establish current report"
	@echo "    commit - compare current with base (terse diff)"
	@echo "    verbose - compare current with base (verbose)"
	@echo "    report - compare current with base"
	@echo "    metrics/current - verbose metrics"
	@echo "    summary/current - summary metrics"

report:
	@echo "----------------------------------"
	@paste *.report

verbose:
	@echo "----------------------------------"
	@sed -e '1,$$s/^.. .[^ ]*.[ ]*//' current.report | paste base.report - | sed -e '1,$$s/^.. //' > $(TMPF)
	@python3 $(BIN)/regression.py $(TMPF)
	@rm -f $(TMPF)

commit:
	@if test -f current.report; then			\
		@echo "----------------------------------"	\
		sed -e '1,$$s/^.. .[^ ]*.[ ]*//' current.report | paste base.report - | sed -e '1,$$s/^.. //' > $(TMPF); \
	    	python3 $(BIN)/commit.py $(TMPF);		\
	    	rm -f $(TMPF);					\
	fi
	python3 $(BIN)/summary-ns.py json/current.json > current.summary

base:
	@rm -f base.report
	@mkdir -p json
	@python3 $(BIN)/stats.py $(NTESTS) > json/base.json
	@python3 $(BIN)/report-stats.py json/base.json > base.report

current:
	@rm -f current.report
	@mkdir -p json
	@python3 $(BIN)/stats.py $(NTESTS) > json/current.json
	@python3 $(BIN)/report-stats.py json/current.json > current.report

metrics/current:
	@for ns in $(NAMESPACES); do				\
	    python3 $(BIN)/metrics-ns.py json/current.json;	\
	done

summary/current:
	@for ns in $(NAMESPACES); do				\
	    python3 $(BIN)/summary-ns.py json/current.json;	\
	done

clean:
	@rm -rf current.report
