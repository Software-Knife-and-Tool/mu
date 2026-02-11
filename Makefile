#
# mu project makefile
#
.PHONY: help world install dist release

help:
	@echo "mu project makefile -----------------"
	@echo "    world - compile for release and build distribution"
	@echo "    release - compile for release"
	@echo "    dist - build distribution (needed for testing debug builds)"
	@echo "    install - install distribution system-wide (may need sudo)"
	@echo "    uninstall - uninstall distribution system-wide (may need sudo)"

world: release dist

dist:
	@make -C dist --no-print-directory

release:
	@cargo build --release --workspace
	@cp target/release/mforge dist
	@cp target/release/mrepl dist
	@cp target/release/mu-exec dist
	@cp target/release/mu-server dist
	@cp target/release/mu-sys dist

install:
	@make -C ./dist -f install.mk install --no-print-directory

uninstall:
	@make -C ./dist -f install.mk uninstall --no-print-directory
