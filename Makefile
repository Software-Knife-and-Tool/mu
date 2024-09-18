#
# mu project makefile
#
.PHONY: world install-mux clean

MUX_DIR = /usr/local/bin

help:
	@echo "mu project makefile -----------------"
	@echo "    world - establish development environment"
	@echo "    install - install mux to system, needs sudo"
	@echo "    $(MUX_DIR) mux install directory - default /usr/local/bin"

world:
	@echo '((nil . ((compile-command . "make -C ~/projects/mu release"))))' > .dir-locals.el
	@etags `find src/mu -name '*.rs' -print`		
	@touch .mu
	@cargo build --release --workspace

install:
	@cp target/release/mux $(MUX_DIR)
