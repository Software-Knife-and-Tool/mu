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

dist:
	@chmod 755 $(DIST)/mu-sys

	@install -d $(BASE)
	@install -d $(BASE)/bin
	@install -d $(BASE)/dist
	@install -d $(BASE)/doc
	@install -d $(BASE)/lib
	@install -d $(BASE)/modules

	@install -m 755 $(SRC)/dist/mu.sh $(BASE)/bin/mu
	@install -m 755 ./mu-exec $(BASE)/bin
	@install -m 755 ./mu-ld $(BASE)/bin
	@install -m 755 ./mu-server $(BASE)/bin
	@install -m 755 ./mu-sys $(BASE)/bin
	@install -m 755 ./mux $(BASE)/bin
	@install -m 755 ./sysgen $(BASE)/bin

	@install -d $(BASE)/doc/html
	@install -m 644 $(DOCS)/refcards/*.pdf $(BASE)/doc
	@install -m 644 $(DOCS)/reference/html/*.html $(BASE)/doc/html

	@install -m 644 $(SRC)/dist/mu.l $(BASE)/dist
	@install -m 644 -D -t $(BASE)/lib/sysgen $(SRC)/sysgen/templates/*
	@install -D -t $(BASE)/lib/common $(SRC)/common/*.l
	@install -D -t $(BASE)/lib/core $(SRC)/core/*.l
	@install -D -t $(BASE)/lib/prelude $(SRC)/prelude/*.l

	@install -D -t $(BASE)/modules/codegen $(SRC)/modules/codegen/*.l

	@make -f dist.mk dist

	@tar --owner=root --group=root -czf $(BASE)-$(VERSION).tgz $(BASE)
	@rm -rf $(BASE)

clean:
	@rm -rf $(BASE)-*.tgz $(BASE) core.l prelude.l mu-exec mu-ld mu-exec mu-server mu-sys mux sysgen
