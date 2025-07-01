#
# mu project makefile
#
.PHONY: world install

MUX_DIR ?= /usr/local/bin

help:
	@echo "mu project makefile -----------------"
	@echo "    world - compile release distrbution"
	@echo "    profile - compile profile distrbution"
	@echo "    install - install distribution system-wide"
	@echo "    (may need sudo)"

world:
	@cargo build --release --workspace
	@cp target/release/mu-exec dist
	@cp target/release/mu-ld dist
	@cp target/release/mu-server dist
	@cp target/release/mu-sh dist
	@cp target/release/mu-sys dist
	@cp target/release/mux dist
	@cp target/release/sysgen dist
	@make -C dist --no-print-directory

profile:
	@cargo build --workspace
	@cp target/release/mu-exec dist
	@cp target/release/mu-ld dist
	@cp target/release/mu-server dist
	@cp target/release/mu-sh dist
	@cp target/release/mu-sys dist
	@cp target/release/mux dist
	@cp target/release/sysgen dist
	@make -C dist --no-print-directory

install:
	@make -C ./dist -f install.mk install --no-print-directory
