#
# mu project makefile
#
.PHONY: world install

MUX_DIR ?= /usr/local/bin

help:
	@echo "mu project makefile -----------------"
	@echo "    world - establish development environment"
	@echo "    install - install mux and mu system-wide, may need sudo"
	@echo "              MUX_DIR - mux install directory, default /usr/local/bin"

world:
	@echo '((nil . ((compile-command . "make -f mu.mk release"))))' > .dir-locals.el
	@etags `find src/mu -name '*.rs' -print`		
	@touch .mu
	@cargo build --release --workspace
	@cp target/release/mu-exec dist
	@cp target/release/mu-ld dist
	@cp target/release/mu-server dist
	@cp target/release/mu-sys dist
	@cp target/release/mux dist
	@cp target/release/sysgen dist
	@make -C dist --no-print-directory

install:
	@make -C ./dist -f install.mk install --no-print-directory
	@cp target/release/mux $(MUX_DIR)
