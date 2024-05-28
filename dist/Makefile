#
# dist makefile
#
.PHONY: dist clean release

VERSION != grep VERSION: ../src/crux/core/lib.rs | sed 's/.* "//' | sed 's/".*//'

BASE = mu
DOCS = ../doc
DIST = ../dist
MU_SYS = ./mu-sys
MU_LD = ./mu-ld
MU_EXEC = ./mu-exec

dist:
	@chmod 755 $(DIST)/mu-sys

	@install -d $(BASE)
	@install -d $(BASE)/bin
	@install -d $(BASE)/doc
	@install -d $(BASE)/lib
	@install -d $(BASE)/lib/prelude
	@install -d $(BASE)/lib/codegen
	@install -d $(BASE)/lib/mu

	@make -f mu.mk mu

	@install -m 644 $(DIST)/prelude.l $(BASE)/lib
	@install -m 644 $(DIST)/mu.l $(BASE)/lib

	@install -m 755 $(MU_EXEC) $(BASE)/bin
	@install -m 755 $(MU_LD) $(BASE)/bin
	@install -m 755 $(MU_SYS) $(BASE)/bin
	@install -m 755 ./mu.sh $(BASE)/bin/mu

	@install -d $(BASE)/doc/html
	@install -m 644 $(DOCS)/refcards/*.pdf $(BASE)/doc
	@install -m 644 $(DOCS)/reference/html/*.html $(BASE)/doc/html

	@tar --owner=root --group=root -czf $(BASE)-$(VERSION).tgz $(BASE)
	@rm -rf $(BASE)

clean:
	@rm -rf $(BASE)-*.tgz $(BASE) $(MU_EXEC) $(MU_LD) $(MU_SYS) $(MU_SERVER) prelude.l
