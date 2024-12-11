#
# tests makefile
#
.PHONY: help cargo mu clean commit common namespaces tests summary core system

TMPF:=$(shell mktemp)
NAMESPACES = mu core common

MODULES = 		\
	core		\
	core/fasl	\
	core/image	\
	common		\
	common/describe	\
	common/metrics	\
	prelude		\
	prelude/repl

help: deltemp
	@echo "mu test makefile -----------------"
	@echo
	@echo "--- test options"
	@echo "    namespaces - list namespaces"
	@echo "    modules - list modules"
	@echo "    commit - create test summary"
	@echo "    test - run single test in \$$NS/\$$TEST"
	@echo "    summary - run all tests in all namespaces and modules and print summary"

deltemp:
	@rm -f $(TMPF)

namespaces: deltemp
	@echo $(NAMESPACES)

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
	@-for namespace in $(NAMESPACES); do				\
	    python3 ./test-ns.py $$namespace . > $(TMPF);		\
	    python3 summarize-ns.py $(TMPF);				\
	done
	@-for module in $(MODULES); do					\
	    python3 ./test-module.py $$module modules > $(TMPF);	\
	    python3 summarize-module.py $(TMPF);			\
	done
	@rm -f $(TMPF)

clean: deltemp
	@rm -f *.diff *.out
