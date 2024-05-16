#
# mu makefile
#
.PHONY: release debug
.PHONY: doc dist install uninstall
.PHONY: clobber commit tags emacs
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
	@echo "    emacs - maintainer's local emacs variables"
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
	@echo "--- footprint options"
	@echo "    footprint/base - baseline report"
	@echo "    footprint/current - current report"
	@echo "    footprint/report - compare base and current"
	@echo "    footprint/commit - condensed footprint report"

tags:
	@etags `find src/libcore -name '*.rs' -print`

emacs: tags
	@echo '((nil . ((compile-command . "make -C ~/projects/mu release"))))' > .dir-locals.el

release:
	@cargo build --release --bin mu-exec
	@cp target/release/mu-exec dist
	@cargo build --release --bin mu-sys
	@cp target/release/mu-sys dist
	@cargo build --release --bin mu-ld
	@cp target/release/mu-ld dist
	@make dist --no-print-directory

debug:
	@cargo build --bin mu-exec
	@cp target/release/mu-exec dist
	@cargo build --bin mu-sys
	@cp target/release/mu-sys dist
	@cargo build --bin mu-ld
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

regression/base:
	@make -C metrics/regression base --no-print-directory

regression/current:
	@make -C metrics/regression current --no-print-directory

regression/report:
	@make -C metrics/regression report --no-print-directory

regression/commit:
	@make -C metrics/regression commit --no-print-directory

footprint/base:
	@make -C metrics/footprint base --no-print-directory

footprint/current:
	@make -C metrics/footprint current --no-print-directory

footprint/report:
	@make -C metrics/footprint report --no-print-directory

footprint/commit:
	@make -C metrics/footprint commit --no-print-directory

commit:
	@cargo fmt
	@echo ";;; internal tests"
	@cargo -q test | sed -e '/^$$/d'
	@echo ";;; clippy tests"
	@cargo clippy
	@echo ";;; external tests report"
	@make -C tests commit --no-print-directory
	@echo ";;; metrics reports"
	@make -C metrics/regression commit --no-print-directory
#	@make -C metrics/footprint commit --no-print-directory

clobber:
	@rm -rf target Cargo.lock TAGS
	@make -C docker clean --no-print-directory
	@make -C dist clean --no-print-directory
	@make -C tests clean --no-print-directory
	@make -C metrics/regression clean --no-print-directory
	@make -C metrics/footprint clean --no-print-directory
