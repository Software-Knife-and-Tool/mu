#
# perf makefile
#
.PHONY: help base clean commit mu current diff frequent summary metrics core

NTESTS ?= 10
TMPF:=$(shell mktemp)
BIN = ./scripts

NAMESPACES=\
	mu	 \
	frequent \
	core

help:
	@echo "mu regression makefile -----------------"
	@echo
	@echo "--- perf options"
	@echo "    namespaces - list namespaces"
	@echo "    list - tests in \$$NS"
	@echo "    \$$NS - run all tests in namespace, unformatted output"
	@echo "    base - run all tests in all namespaces, establish baseline report"
	@echo "    current - run all tests in all namespace, establish current report"
	@echo "    commit - compare current with base (terse diff), promote current to base"
	@echo "    regression - compare current with base (verbose)"
	@echo "    report - compare current with base"
	@echo "    metrics/current - verbose metrics"
	@echo "    summary/current - summary metrics"

report:
	@sed -e '1,$$s/^.. .[^ ]*.[ ]*//' current.report | paste base.report - | sed -e '1,$$s/^.. //' > $(TMPF)
	@python3 $(BIN)/report.py $(TMPF)
	@rm -f $(TMPF)

regression:
	@sed -e '1,$$s/^.. .[^ ]*.[ ]*//' current.report | paste base.report - | sed -e '1,$$s/^.. //' > $(TMPF)
	@python3 $(BIN)/regression.py $(TMPF)
	@rm -f $(TMPF)

commit:
	@if test -f current.report; then	\
	    sed -e '1,$$s/^.. .[^ ]*.[ ]*//' current.report | paste base.report - | sed -e '1,$$s/^.. //' > $(TMPF); \
	    python3 $(BIN)/commit.py $(TMPF);	\
	    rm -f $(TMPF);			\
	    mv current.report base.report;	\
	fi
	@rm -f current.summary
	@for ns in $(NAMESPACES); do							\
	    python3 $(BIN)/summary-ns.py json/current.$$ns.json >> current.summary;	\
	done

mu:
	@python3 $(BIN)/perf-ns.py mu . $(NTESTS)

frequent:
	@python3 $(BIN)/perf-ns.py frequent . $(NTESTS)	

core:
	@python3 $(BIN)/perf-ns.py core . $(NTESTS)	

namespaces:
	@echo $(NAMESPACES)

list:
	@cat $$NS/tests

base:
	@rm -f base.report
	@mkdir -p json
	@for ns in $(NAMESPACES); do						\
	    make -s $$ns > json/base.$$ns.json;					\
	    python3 $(BIN)/report-ns.py json/base.$$ns.json >> base.report;	\
	done

current:
	@rm -f current.report
	@mkdir -p json
	@for ns in $(NAMESPACES); do							\
	    make -s $$ns > json/current.$$ns.json;					\
	    python3 $(BIN)/report-ns.py json/current.$$ns.json>> current.report;	\
	done

metrics/current:
	@for ns in $(NAMESPACES); do					\
	    python3 $(BIN)/metrics-ns.py json/current.$$ns.json;	\
	done

summary/current:
	@for ns in $(NAMESPACES); do					\
	    python3 $(BIN)/summary-ns.py json/current.$$ns.json;	\
	done

clean:
	@rm -rf json current.report
