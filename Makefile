#
# mu makefile
#
.PHONY: release debug
.PHONY: doc dist install uninstall
.PHONY: clobber commit tags
.PHONY: tests/rust tests/summary tests/report
.PHONY: regression/base regression/current regression/report regression/commit

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
	@echo "--- regression options"
	@echo "    regression/base - baseline report"
	@echo "    regression/current - current report"
	@echo "    regression/report - compare base and current"
	@echo "    regression/commit - condensed regression report"

tags:
	@etags `find src/mu -name '*.rs' -print`

release:
	@cargo build --release --bin mu-exec
	@cp target/release/mu-exec dist
	@cargo build --release --bin mu-sys
	@cp target/release/mu-sys dist
	@cargo build --release --bin mu-ld
	@cp target/release/mu-ld dist
	@make dist --no-print-directory

debug:
	@cargo build --bin mu-sys
	@cp target/debug/mu-sys dist
	@cargo build --release --bin mu-ld
	@cp target/debug/mu-ld dist
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

regression/base:
	@make -C metrics/regression base --no-print-directory

regression/current:
	@make -C metrics/regression current --no-print-directory

regression/report:
	@make -C metrics/regression report --no-print-directory

regresssion/commit:
	@make -C metrics/regression commit --no-print-directory

commit:
	@cargo fmt
	@echo ";;; rust tests"
	@cargo -q test | sed -e '/^$$/d'
	@echo ";;; clippy tests"
	@cargo clippy
	@echo ";;; regression report"
	@make -C tests commit --no-print-directory
	@make -C metrics/regression commit --no-print-directory

clobber:
	@rm -f TAGS
	@rm -rf target Cargo.lock
	@make -C docker clean --no-print-directory
	@make -C dist clean --no-print-directory
	@make -C tests clean --no-print-directory
	@make -C metrics/regression clean --no-print-directory
