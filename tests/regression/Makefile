#
# tests makefile
#
.PHONY: help cargo mu clean commit namespaces tests summary core system

TMPF:=$(shell mktemp)
NAMESPACES = mu core

help:
	@echo "mu test makefile -----------------"
	@echo
	@echo "--- test options"
	@echo "    cargo - run rust tests"
	@echo "    namespaces - list namespaces"
	@echo "    commit - create test summary"
	@echo "    tests - tests in \$$NS"
	@echo "    $(NAMESPACES) - run all tests in namespace, raw output"
	@echo "    test - run single test in \$$NS/\$$TEST"
	@echo "    summary - run all tests in all namespaces and print summary"

cargo:
	@cargo test | grep "test result"

namespaces:
	@echo $(NAMESPACES)

list:
	@cat $$NS/tests

mu:
	@python3 ./test-ns.py mu .

core:
	@python3 ./test-ns.py core .	

commit:
	@make summary --no-print-directory > tests.summary
	@echo "Test Summaries"
	@echo "--------------"
	@grep -v "failed: 0        aborted: 0" tests.summary | grep -v -- "----" | grep -v "Test Summary"
	@echo "Test Diffs"
	@echo "-------------"
	@git diff --unified=0 tests.summary

test:
	@python3 test.py $$NS . $$TEST > $(TMPF)
	@python3 summarize-test.py $(TMPF)
	@rm -f $(TMPF)

summary:
	@for namespace in $(NAMESPACES); do			\
	    make $$namespace --no-print-directory > $(TMPF);	\
	    python3 summarize-ns.py $(TMPF);			\
	done
	@rm -f $(TMPF)

clean:
	@rm -f *.diff *.out
