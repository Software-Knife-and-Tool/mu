#
# tests makefile
#
.PHONY: help cargo mu clean commit tests summary system

TMPF:=$(shell mktemp)

# MODULES = 		\
#	core		\
#	common		\
#	common/describe	\
#	common/metrics	\
#	prelude		\
#	prelude/repl

MODULES = core module

help: deltemp
	@echo "regression test makefile -----------------"
	@echo
	@echo "--- test options"
	@echo "    commit - create test summary"
	@echo "    test - run single test group in \$$NS/\$$TEST"
	@echo "    summary - run all tests in all namespaces and print summary"

deltemp:
	@rm -f $(TMPF)

modules: deltemp
	@echo $(MODULES)

commit: deltemp
	@make summary --no-print-directory > tests.summary
	@echo "Test Summaries"
	@echo "--------------"
	@grep -v "failed: 0        aborted: 0" tests.summary | grep -v -- "----" | grep -v "Test Summary"
	@echo "Test Diffs"
	@echo "-------------"
	@git diff --unified=0 tests.summary

test: deltemp
	@-python3 test.py $$NS . $$TEST > $(TMPF)
	@-python3 summarize-test.py $(TMPF)

summary:
	@python3 ./test-ns.py mu namespaces > $(TMPF);
	@python3 summarize-ns.py $(TMPF);
	@-for module in $(MODULES); do					\
	    python3 ./test-module.py $$module namespaces > $(TMPF);	\
	    python3 summarize-module.py $(TMPF);			\
	done

	@rm -f $(TMPF)

clean: deltemp
	@rm -f *.diff *.out *.fasl
