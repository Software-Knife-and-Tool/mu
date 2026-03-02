#
# install makefile
#
.PHONY: install release uninstall help

VERSION = `cat ./VERSION`

ROOT = /opt
BASE = system-lisp

help:
	@echo install - install $(BASE) in $(ROOT)/$(BASE) (needs sudo)
	@echo uninstall - remove $(BASE) from $(ROOT) (needs sudo)

install:
	@cat ./$(VERSION).tgz | (cd $(ROOT); tar --no-same-owner -xzf -)

uninstall:
	@rm -rf $(ROOT)/$(BASE)
