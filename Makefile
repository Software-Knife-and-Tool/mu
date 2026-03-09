#
# mu project makefile
#
.PHONY: help world install dist release emacs tests/base tests/regression tests/current tests/report install uninstall

help:
	@echo "mu project makefile -----------------"
	@echo "    world - compile for release and build distribution"
	@echo "    release - compile for release"
	@echo "    dist - build distribution (needed for testing debug builds)"
	@echo "    tests/regression - run regression tests"
	@echo "    tests/base - run performance base tests"
	@echo "    tests/current - run performance current tests"
	@echo "    tests/report - prinmt bnase vs current report"
	@echo "    install - install distribution system-wide (may need sudo)"
	@echo "    uninstall - uninstall distribution system-wide (may need sudo)"

world: release dist

dist:
	@make -C dist TARGET_FEATURE="" --no-print-directory
release:
	@cargo build --release --workspace

emacs:
	@echo '((nil . ((compile-command . "cd ~/projects/system-lisp ; make world"))))' > .dir-locals.el
	@find src -name "*.rs" -print | etags -

tests/regression:
	@make -C tests/regression summary
tests/base:
	@make -C tests/performance base
tests/current:
	@make -C tests/performance current
tests/report:
	@make -C tests/performance report

install:
	@make -C ./dist -f install.mk install --no-print-directory
uninstall:
	@make -C ./dist -f install.mk uninstall --no-print-directory
