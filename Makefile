#
# mu makefile
#
.PHONY: release debug
.PHONY: doc dist install uninstall
.PHONY: clobber commit tags
.PHONY: tests/rust tests/summary tests/report
.PHONY: perf/base perf/current perf/diff perf/commit

help:
	@echo "mu top-level makefile -----------------"
	@echo
	@echo "--- build options"
	@echo "    debug - build runtime for debug and package for distribution"
	@echo "    release - build runtime for release and package for distribution"
	@echo "--- distribution options"
	@echo "    doc - generate documentation"
	@echo "    dist - build distribution image"
	@echo "    install - install distribution (needs sudo)"
	@echo "    uninstall - uninstall distribution (needs sudo)"
	@echo "--- development options"
	@echo "    clobber - remove build artifacts"
	@echo "    commit - run clippy, rustfmt, make test and perf reports"
	@echo "    tags - make etags"
	@echo "--- test options"
	@echo "    tests/rust - rust tests"
	@echo "    tests/summary - test summary"
	@echo "    tests/report - full test report"
	@echo "--- perf options"
	@echo "    perf/base - baseline report"
	@echo "    perf/current - current report"
	@echo "    perf/diff - compare base and current"
	@echo "    perf/commit - condensed report"

tags:
	@etags `find src/mu -name '*.rs' -print`

release:
	@cargo build --release --bin mu-exec
	@cp target/release/mu-exec dist
	@cargo build --release --bin mu-sh
	@cp target/release/mu-sh dist
	@cargo build --release --bin mu-ld
	@cp target/release/mu-ld dist
	@make dist --no-print-directory

debug:
	@cargo build --bin mu-sh
	@cp target/debug/mu-sh dist
	@cargo build --release --bin mu-ld
	@cp target/release/mu-ld dist
	@make dist --no-print-directory

dist:
	@make -C ./dist --no-print-directory

doc:
	@cargo doc
	@mkdir -p ./doc/rustdoc
	@cp -r ./target/doc/* ./doc/rustdoc
	@make -C ./doc --no-print-directory

install:
	@make -C ./dist -f install.mk install --no-print-directory

uninstall:
	@make -C ./dist -f install.mk uninstall --no-print-directory

tests/commit:
	@make -C tests commit --no-print-directory

tests/summary:
	@make -C tests summary --no-print-directory

perf/base:
	@make -C perf base --no-print-directory

perf/current:
	@make -C perf current --no-print-directory

perf/diff:
	@make -C perf diff --no-print-directory

perf/commit:
	@make -C perf commit --no-print-directory

commit:
	@cargo fmt
	@echo ";;; rust tests"
	@cargo -q test | sed -e '/^$$/d'
	@echo ";;; clippy tests"
	@cargo clippy
	@make -C tests commit --no-print-directory
	@make -C perf commit --no-print-directory

clobber:
	@rm -f TAGS
	@rm -rf target Cargo.lock
	@make -C docker clean --no-print-directory
	@make -C dist clean --no-print-directory
	@make -C tests clean --no-print-directory
	@make -C perf clean --no-print-directory
