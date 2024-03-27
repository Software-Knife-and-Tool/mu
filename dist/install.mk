#
# install makefile
#
.PHONY: install release uninstall help

ROOT = /opt
BASE = mu

help:
	@echo install - install $(BASE) in $(ROOT)/$(BASE) (needs sudo)
	@echo uninstall - remove $(BASE) from $(ROOT) (needs sudo)

install:
	@cat ./$(BASE)*.tgz | (cd $(ROOT); tar --no-same-owner -xzf -)

uninstall:
	@rm -rf $(ROOT)/$(BASE)
