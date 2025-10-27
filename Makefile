#
# mu project makefile
#
.PHONY: help world install dist release debug

help:
	@echo "mu project makefile -----------------"
	@echo "    world - compile for release and build distribution"
	@echo "    release - compile for release"
	@echo "    debug - compile for debug"
	@echo "	   dist - build distribution (needed for testing debug builds)"
	@echo "    install - install distribution system-wide"
	@echo "    (may need sudo)"

world: release dist

dist:
	@make -C dist --no-print-directory

release:
	@cargo build --release --workspace
	@cp target/release/forge dist
	@cp target/release/listener dist
	@cp target/release/mu-exec dist
	@cp target/release/mu-server dist
	@cp target/release/mu-sys dist

debug:
	@cargo build --workspace
	@cp target/debug/forge dist
	@cp target/debug/listener dist
	@cp target/debug/mu-exec dist
	@cp target/debug/mu-server dist
	@cp target/debug/mu-sys dist

install:
	@make -C ./dist -f install.mk install --no-print-directory
