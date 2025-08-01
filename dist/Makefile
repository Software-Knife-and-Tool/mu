#
# dist makefile
#
.PHONY: dist clean release

VERSION != grep VERSION: ../src/mu/core/core.rs | sed 's/.* "//' | sed 's/".*//'

BASE = ./mu
DIST = ../dist
DOCS = ../doc
LIB = ../lib
SRC = ../src
TOOLS = ../tools

MODULES = 		\
	common		\
	common/describe	\
	common/metrics	\
	prelude		\
	prelude/repl

dist:
	@install -d $(BASE)
	@install -d $(BASE)/bin
	@install -d $(BASE)/doc
	@install -d $(BASE)/lib
	@install -d $(BASE)/modules

	@install -m 755 $(SRC)/dist/mu.sh $(BASE)/bin/mu
	@install -m 555 $(SRC)/dist/annotate.py $(BASE)/bin/annotate.py

	@install -m 755 ./mu-exec $(BASE)/bin
	@install -m 755 ./mu-server $(BASE)/bin
	@install -m 755 ./mu-sh $(BASE)/bin
	@install -m 755 ./mu-sys $(BASE)/bin
	@install -m 755 ./mux $(BASE)/bin
	@install -m 755 ./mcore $(BASE)/bin
	@install -m 755 ./sysgen $(BASE)/bin

	@install -d $(BASE)/doc/html
	@install -m 644 $(DOCS)/refcards/*.pdf $(BASE)/doc
	@install -m 644 $(DOCS)/reference/html/*.html $(BASE)/doc/html

	@install -m 644 $(SRC)/dist/mu.l $(BASE)/lib
	@install -m 644 $(SRC)/dist/image.l $(BASE)/lib

	@install -m 644 -D -t $(BASE)/lib/sysgen $(SRC)/sysgen/templates/*
	@install -m 644 -D -t $(BASE)/lib/mcore $(SRC)/mcore/lib/*

	@install -D -m 644 -t $(BASE)/lib/core $(SRC)/core/*.l
	@install -m 644 -t $(BASE)/lib/core $(SRC)/core/*.def

	@install -D -m 644 -t $(BASE)/lib/core/fasl $(SRC)/core/fasl/*.l
	@install -m 644 -t $(BASE)/lib/core/fasl $(SRC)/core/fasl/*.def

	@install -D -m 644 -t $(BASE)/lib/core/image $(SRC)/core/image/*.l
	@install -m 644 -t $(BASE)/lib/core/image $(SRC)/core/image/*.def

	@for module in $(MODULES); do	\
	    install -D -m 644 -t $(BASE)/modules/$$module $(SRC)/modules/$$module/*.l;		\
	    install -D -m 644 -t $(BASE)/modules/$$module $(SRC)/modules/$$module/mod.def;	\
	done

	@./mu-sys -l ../src/dist/fasl.l -q '(fasl:require "../src/" "core")'
	@install -m 644 ./core.fasl $(BASE)/lib

	@./mu-sys -l ../src/dist/fasl.l -l ./core.fasl -q '(fasl:require "../src/modules/" "common")'
	@install -m 644 ./common.fasl $(BASE)/lib

	@tar --owner=root --group=root -czf $(BASE)-$(VERSION).tgz $(BASE)
	@rm -rf $(BASE) mu-exec mu-server mu-sh mcore mux sysgen

clean:
	@rm -rf $(BASE)-*.tgz $(BASE) core.fasl mcore mu-exec mu-server mu-sh mu-sys mux sysgen
