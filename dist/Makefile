#
# dist makefile
#
.PHONY: dist clean release

VERSION != grep VERSION: ../src/mu/core/lib.rs | sed 's/.* "//' | sed 's/".*//'

BASE = mu
DIST = ../dist
DOCS = ../doc
SRC = ../src

dist:
	@chmod 755 $(DIST)/mu-sys

	@install -d $(BASE)
	@install -d $(BASE)/bin
	@install -d $(BASE)/doc
	@install -d $(BASE)/lib
	@install -d $(BASE)/lib/codegen
	@install -d $(BASE)/lib/mu
	@install -d $(BASE)/lib/prelude
	@install -d $(BASE)/lib/sysgen

	@make -f mu.mk mu

	@install -m 644 $(DIST)/prelude.l $(BASE)/lib
	@install -m 644 $(SRC)/lib/mu.l $(BASE)/lib

	@install -m 755 ./sysgen $(BASE)/bin
	@install -m 644 -t $(BASE)/lib/sysgen $(SRC)/sysgen/templates/*

	@install -m 755 ./mu-exec $(BASE)/bin
	@install -m 755 ./mu-ld $(BASE)/bin
	@install -m 755 ./mu-sys $(BASE)/bin

	@install -m 755 $(SRC)/bin/mu.sh $(BASE)/bin/mu

	@install -d $(BASE)/doc/html
	@install -m 644 $(DOCS)/refcards/*.pdf $(BASE)/doc
	@install -m 644 $(DOCS)/reference/html/*.html $(BASE)/doc/html

	@tar --owner=root --group=root -czf $(BASE)-$(VERSION).tgz $(BASE)
	@rm -rf $(BASE)

clean:
	@rm -rf $(BASE)-*.tgz $(BASE) mu-exec mu-ld mu-exec sysgen prelude.l
