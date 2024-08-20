#
# mu makefile
#
.PHONY: release debug
.PHONY: doc dist install uninstall
.PHONY: clobber commit tags emacs
.PHONY: tests/rust tests/base tests/current tests/report
.PHONY: regression/base regression/current regression/report regression/commit

help:
	@echo "mu top-level makefile -----------------"
	@echo
	@echo "--- build options"
	@echo "    debug - build runtime for debug and package for distribution"
	@echo "    release - build runtime for release and package for distribution"
	@echo "    perf - build runtime for performance monitoring and package for distribution"
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
	@echo "    tests/base - everything (may take a while)"
	@echo "    tests/current - full test report"
	@echo "    tests/report - condensed performance report"
	@echo "    tests/commit - establish what needs to go into repo"
	@echo "--- regression options"
	@echo "    regression/summary - test summary"
	@echo "    regression/report - full test report"
	@echo "--- performance options"
	@echo "    performance/base - baseline report"
	@echo "    performance/current - current report"
	@echo "    performance/report - compare base and current"
	@echo "--- footprint options"
	@echo "    footprint/base - baseline report"
	@echo "    footprint/current - current report"
	@echo "    footprint/report - compare base and current"

tags:
	@etags `find src/mu -name '*.rs' -print`

emacs: tags
	@echo '((nil . ((compile-command . "make -C ~/projects/mu release"))))' > .dir-locals.el

release:
	@cargo build --release --workspace
	@cp target/release/mu-exec dist
	@cp target/release/mu-ld dist
	@cp target/release/mu-server dist
	@cp target/release/mu-sys dist
	@cp target/release/mux dist
	@cp target/release/sysgen dist
	@make dist --no-print-directory

debug:
	@cargo build --workspace
	@cp target/release/mu-exec dist
	@cp target/release/mu-ld dist
	@cp target/release/mu-server dist
	@cp target/release/mu-sys dist
	@cp target/release/mux dist
	@cp target/release/sysgen dist
	@make dist --no-print-directory

perf:
	@cargo build --profile perf --features perf --workspace
	@cp target/perf/devop dist
	@cp target/perf/mu-exec dist
	@cp target/perf/mu-ld dist
	@cp target/perf/mu-server dist
	@cp target/perf/mux dist
	@cp target/perf/sysgen dist
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

tests/base: performance/base footprint/base

tests/current: performance/current footprint/current

tests/report: regression/report performance/report footprint/report

regression/commit:
	@make -C tests/regression commit --no-print-directory

regression/report:
	@make -C tests/regression summary --no-print-directory

performance/base:
	@make -C tests/performance base --no-print-directory

performance/current:
	@make -C tests/performance current --no-print-directory

performance/report:
	@make -C tests/performance report --no-print-directory

performance/commit:
	@make -C tests/regression commit --no-print-directory

footprint/base:
	@make -C tests/footprint base --no-print-directory

footprint/current:
	@make -C tests/footprint current --no-print-directory

footprint/report:
	@make -C tests/footprint report --no-print-directory

footprint/commit:
	@make -C tests/footprint commit --no-print-directory

commit:
	@cargo fmt
	@echo ";;; internal tests"
	@cargo -q test | sed -e '/^$$/d'
	@echo ";;; clippy tests"
	@cargo clippy
	@echo ";;; external tests report"
	@make -C tests/regression commit --no-print-directory
	@echo ";;;  reports"
	@make -C tests/performance commit --no-print-directory
#	@make -C tests/footprint commit --no-print-directory

clobber:
	@rm -rf target Cargo.lock TAGS
	@make -C docker clean --no-print-directory
	@make -C dist clean --no-print-directory
	@make -C tests/regression clean --no-print-directory
	@make -C tests/footprint clean --no-print-directory
