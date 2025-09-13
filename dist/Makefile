#
# dist makefile
#
.PHONY: dist clean release

VERSION := $(shell cargo pkgid | sed 's/.*#//')

BASE = ./mu
DIST = ../dist
DOCS = ../doc
MU = ../mu
SRC = ../src
TOOLS = ../tools

BIN = mu-exec mu-server mu-listener manifest
SYS = core.sys fasl.sys image.sys

MODULES = 		\
	common		\
	common/describe	\
	common/metrics	\
	prelude		\
	prelude/repl

dist:
	@echo $(VERSION) > VERSION

	@install -d $(BASE)	
	@install -d $(BASE)/bin
	@install -d $(BASE)/doc
	@install -d $(BASE)/lib
	@install -d $(BASE)/modules

	@for bin in $(BIN); do			\
	    install -m 755 $$bin $(BASE)/bin;	\
	done

	@install -d $(BASE)/doc/html
	@install -m 644 $(DOCS)/refcards/*.pdf $(BASE)/doc
	@install -m 644 $(DOCS)/reference/html/*.html $(BASE)/doc/html

	@install -m 644 $(MU)/dist/mu.l $(BASE)/lib
	@install -m 644 $(MU)/dist/image.l $(BASE)/lib

	@install -m 644 -D -t $(BASE)/lib/mu-listener $(SRC)/mu-listener/lib/*

	@install -D -m 644 -t $(BASE)/lib/core $(MU)/sys/core/*.l
	@install -m 644 -t $(BASE)/lib/core $(MU)/sys/core/*.def
	@install -D -m 644 -t $(BASE)/lib/fasl $(MU)/sys/fasl/*.l
	@install -m 644 -t $(BASE)/lib/fasl $(MU)/sys/fasl/*.def
	@install -D -m 644 -t $(BASE)/lib/image $(MU)/sys/image/*.l
	@install -m 644 -t $(BASE)/lib/image $(MU)/sys/image/*.def

	@for module in $(MODULES); do	\
	    install -D -m 644 -t $(BASE)/modules/$$module $(MU)/modules/$$module/*.l;		\
	    install -D -m 644 -t $(BASE)/modules/$$module $(MU)/modules/$$module/mod.def;	\
	done

	@./mu-sys -l $(MU)/dist/system.l -q '(system:require "../mu/sys/" "core")'
	@install -m 644 ./core.sys $(BASE)/lib

	@./mu-sys -l $(MU)/dist/fasl.l -l ./core.sys -q '(fasl:require "../mu/modules/" "common")'
	@install -m 644 ./common.fasl $(BASE)/lib

	@tar --owner=root --group=root -czf $(BASE)-$(VERSION).tgz $(BASE)
	@rm -rf $(BASE) $(BIN)

clean:
	@rm -rf $(BASE)-*.tgz $(BASE) core.fasl $(BIN)
