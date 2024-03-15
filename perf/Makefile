#
# perf makefile
#
.PHONY: help base clean commit current diff frequent summary metrics mu prelude

NTESTS ?= 20
TMPF:=$(shell mktemp)
NAMESPACES=\
	mu	 \
	frequent \
	prelude

VALGRIND='((:lambda (g) (mu:apply g (mu:cons g (mu:cons 101010101 (mu:cons 11011 ()))))) (:lambda (g a b) (:if (mu:eq 0 b) a (mu:apply g (mu:cons g (mu:cons b (mu:cons (mu:fx-sub a (mu:fx-mul b (mu:fx-div a b))) ()))))))) '

help:
	@echo "mu perf makefile -----------------"
	@echo
	@echo "--- perf options"
	@echo "    namespaces - list namespaces"
	@echo "    list - tests in \$$NS"
	@echo "    \$$NS - run all tests in namespace, unformatted output"
	@echo "    base - run all tests in all namespaces, establish baseline report"
	@echo "    current - run all tests in all namespace, establish current report"
	@echo "    commit - compare current with base (terse diff), promote current to base"
	@echo "    diff - compare current with base (terse diff)"
	@echo "    full-diff - compare current with base (verbose diff)"
	@echo "    metrics/current - verbose metrics"
	@echo "    summary/current - summary metrics"
	@echo "    valgrind - run memcheck, callgrind, cachegrind reports"

valgrind: memcheck callgrind cachegrind clean

diff:
	@sed -e '1,$$s/^.. .[^ ]*.[ ]*//' current.report | paste base.report - | sed -e '1,$$s/^.. //' > $(TMPF)
	@python3 commit.py $(TMPF)
	@rm -f $(TMPF)

full-diff:
	@sed -e '1,$$s/^.. .[^ ]*.[ ]*//' current.report | paste base.report - | sed -e '1,$$s/^.. //' > $(TMPF)
	@python3 diff.py $(TMPF)
	@rm -f $(TMPF)

commit:
	@if test -f current.report; then	\
	    sed -e '1,$$s/^.. .[^ ]*.[ ]*//' current.report | paste base.report - | sed -e '1,$$s/^.. //' > $(TMPF); \
	    python3 commit.py $(TMPF);		\
	    rm -f $(TMPF);			\
	    mv current.report base.report;	\
	fi
	@rm -f current.summary
	@for ns in $(NAMESPACES); do						\
	    python3 summary-ns.py json/current.$$ns.json >> current.summary;	\
	done

mu:
	@python3 ./perf-ns.py mu . $(NTESTS)

frequent:
	@python3 ./perf-ns.py frequent . $(NTESTS)	

prelude:
	@python3 ./perf-ns.py prelude . $(NTESTS)	

namespaces:
	@echo $(NAMESPACES)

list:
	@cat $$NS/tests

base:
	@rm -f base.report
	@mkdir -p json
	@for ns in $(NAMESPACES); do					\
	    make -s $$ns > json/base.$$ns.json;				\
	    python3 report-ns.py json/base.$$ns.json >> base.report;	\
	done

current:
	@rm -f current.report
	@mkdir -p json
	@for ns in $(NAMESPACES); do						\
	    make -s $$ns > json/current.$$ns.json;				\
	    python3 report-ns.py json/current.$$ns.json>> current.report;	\
	done

metrics/current:
	@for ns in $(NAMESPACES); do				\
	    python3 metrics-ns.py json/current.$$ns.json;	\
	done

summary/current:
	@for ns in $(NAMESPACES); do				\
	    python3 summary-ns.py json/current.$$ns.json;	\
	done

memcheck:
	@valgrind --tool=memcheck --leak-check=yes --show-reachable=yes --num-callers=10 \
		--leak-resolution=med --show-leak-kinds=definite \
		../dist/mu-sh -p      \
		    -q $(VALGRIND) > memcheck.report 2>&1 || true

callgrind:
	@valgrind --tool=callgrind     	\
		../dist/mu-sh -p      \
		    -q $(VALGRIND) > callgrind.report 2>&1 || true

	@callgrind_annotate --auto=yes callgrind.out.* >> callgrind.report

cachegrind:
	@valgrind --tool=cachegrind	\
		../dist/mu-sh -p      \
		    -q $(VALGRIND) > cachegrind.report 2>&1 || true

clean:
	@rm -rf json current.report
