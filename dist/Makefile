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
	@install -d $(BASE)/doc
	@install -d $(BASE)/lib
	@install -d $(BASE)/lib/codegen
	@install -d $(BASE)/lib/common
	@install -d $(BASE)/lib/mu
	@install -d $(BASE)/lib/prelude
	@install -d $(BASE)/lib/prelude/core
	@install -d $(BASE)/lib/prelude/system
	@install -d $(BASE)/lib/prelude/repl
	@install -d $(BASE)/lib/prelude/inspect
	@install -d $(BASE)/lib/sysgen

	@make -f dist.mk dist

	@install -m 644 $(SRC)/dist/mu.l $(BASE)/lib
	@install -m 644 -t $(BASE)/lib/sysgen $(SRC)/sysgen/templates/*

	@install -m 755 ./sysgen $(BASE)/bin
	@install -m 755 ./mu-exec $(BASE)/bin
	@install -m 755 ./mu-ld $(BASE)/bin
	@install -m 755 ./mu-server $(BASE)/bin
	@install -m 755 ./mu-sys $(BASE)/bin

	@install -m 755 $(SRC)/dist/mu.sh $(BASE)/bin/mu

	@install -d $(BASE)/doc/html
	@install -m 644 $(DOCS)/refcards/*.pdf $(BASE)/doc
	@install -m 644 $(DOCS)/reference/html/*.html $(BASE)/doc/html

	@tar --owner=root --group=root -czf $(BASE)-$(VERSION).tgz $(BASE)
	@rm -rf $(BASE)

clean:
	@rm -rf $(BASE)-*.tgz $(BASE) mu-exec mu-ld mu-exec mu-server mu-sys sysgen prelude
