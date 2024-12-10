#
# tests makefile
#
.PHONY: help cargo mu clean commit common \
        namespaces tests summary core system \
	core/core core/fasl core/image common/common	\
	common/describe	common/metrics prelude/prelude	\
	prelude/repl

TMPF:=$(shell mktemp)
NAMESPACES = mu core common

MODULES = 		\
	core/core	\
	core/fasl	\
	core/image	\
	common/common	\
	common/describe	\
	common/metrics	\
	prelude/prelude	\
	prelude/repl

help:
	@echo "mu test makefile -----------------"
	@echo
	@echo "--- test options"
	@echo "    namespaces - list namespaces"
	@echo "    modules - list modules"
	@echo "    commit - create test summary"
	@echo "    $(NAMESPACES) - run all tests in namespace, raw output"
	@echo "    $(MODULES) - run all tests in module, raw output"
	@echo "    test - run single test in \$$NS/\$$TEST"
	@echo "    summary - run all tests in all namespaces and modules and print summary"

namespaces:
	@echo $(NAMESPACES)

modules:
	@echo $(MODULES)

$(NAMESPACES):
	@python3 ./test-ns.py $@ .

$(MODULES):
	@python3 ./test-module.py $@ modules

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
	@for module in $(MODULES); do				\
	    make $$module --no-print-directory > $(TMPF);	\
	    python3 summarize-module.py $(TMPF);		\
	done
	@rm -f $(TMPF)

clean:
	@rm -f *.diff *.out
