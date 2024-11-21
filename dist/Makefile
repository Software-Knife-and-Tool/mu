#
# dist makefile
#
.PHONY: dist clean release

VERSION != grep VERSION: ../src/mu/core/lib.rs | sed 's/.* "//' | sed 's/".*//'

BASE = ./mu
DIST = ../dist
DOCS = ../doc
LIB = ../lib
SRC = ../src
TOOLS = ../tools

dist:
	@chmod 755 $(DIST)/mu-sys
	@chmod 755 $(DIST)/mu-sh

	@install -d $(BASE)
	@install -d $(BASE)/bin
	@install -d $(BASE)/dist
	@install -d $(BASE)/doc
	@install -d $(BASE)/lib
	@install -d $(BASE)/modules

	@install -m 755 $(SRC)/dist/mu.sh $(BASE)/bin/mu
	@install -m 555 $(SRC)/dist/annotate.py $(BASE)/bin/annotate.py

	@install -m 755 ./mu-exec $(BASE)/bin
	@install -m 755 ./mu-ld $(BASE)/bin
	@install -m 755 ./mu-server $(BASE)/bin
	@install -m 755 ./mu-sh $(BASE)/bin
	@install -m 755 ./mu-sys $(BASE)/bin
	@install -m 755 ./sysgen $(BASE)/bin

	@install -d $(BASE)/doc/html
	@install -m 644 $(DOCS)/refcards/*.pdf $(BASE)/doc
	@install -m 644 $(DOCS)/reference/html/*.html $(BASE)/doc/html

	@install -m 644 $(SRC)/dist/mu.l $(BASE)/dist
	@install -m 644 -D -t $(BASE)/lib/sysgen $(TOOLS)/sysgen/templates/*
	@install -m 644 -D -t $(BASE)/lib/core $(SRC)/core/*.l
	@install -m 644 -D -t $(BASE)/lib/prelude $(SRC)/modules/prelude/*.l

	@install -D -t $(BASE)/modules/codegen $(SRC)/modules/codegen/*.l
	@install -D -t $(BASE)/modules/common $(SRC)/modules/common/*.l
	@install -D -t $(BASE)/modules/repl $(SRC)/modules/repl/*.l
	@install -D -t $(BASE)/modules/prelude $(SRC)/modules/prelude/*.l

	@make -f dist.mk dist

	@tar --owner=root --group=root -czf $(BASE)-$(VERSION).tgz $(BASE)
	@rm -rf $(BASE)

clean:
	@rm -rf $(BASE)-*.tgz $(BASE) core.l prelude.l mu-exec mu-ld mu-exec mu-server mu-sh mu-sys sysgen
