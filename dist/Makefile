#
# dist makefile
#
.PHONY: dist clean release

VERSION != grep VERSION: ../src/mu/core/lib.rs | sed 's/.* "//' | sed 's/".*//'

BASE = mu
DOCS = ../doc
DIST = ../dist

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

	@install -m 755 ./mu-bindgen $(BASE)/bin
	@install -m 755 ./mu-exec $(BASE)/bin
	@install -m 755 ./mu-ld $(BASE)/bin
	@install -m 755 ./mu-sys $(BASE)/bin
	@install -m 755 ./mu.sh $(BASE)/bin/mu

	@install -d $(BASE)/doc/html
	@install -m 644 $(DOCS)/refcards/*.pdf $(BASE)/doc
	@install -m 644 $(DOCS)/reference/html/*.html $(BASE)/doc/html

	@tar --owner=root --group=root -czf $(BASE)-$(VERSION).tgz $(BASE)
	@rm -rf $(BASE)

clean:
	@rm -rf $(BASE)-*.tgz $(BASE) $(MU_EXEC) $(MU_LD) $(MU_SYS) $(MU_SERVER) prelude.l
