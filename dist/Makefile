#
# dist makefile
#
.PHONY: dist clean

VERSION = 0.0.37

BASE = mu
DOCS = ../doc
DIST = ../dist
MU_SH = ./mu-sh
MU_LD = ./mu-ld
MU_EXEC = ./mu-exec

dist:
	@chmod 755 $(DIST)/mu-sh

	@install -d $(BASE)
	@install -d $(BASE)/mu
	@install -d $(BASE)/mu/prelude
	@install -d $(BASE)/mu/mu
	@install -d $(BASE)/mu/codegen

	@make -f mu.mk mu prelude

	@install -d $(BASE)/bin
	@install -d $(BASE)/$(BASE)
	@install -d $(BASE)/doc

	@install -m 644 $(DIST)/mu.l $(BASE)/$(BASE)

	@install -m 755 $(MU_EXEC) $(BASE)/bin
	@install -m 755 $(MU_LD) $(BASE)/bin
	@install -m 755 $(MU_SH) $(BASE)/bin
	@install -m 755 ./mu.sh $(BASE)/bin/mu

	@install -d $(BASE)/doc/html
	@install -m 644 $(DOCS)/refcards/*.pdf $(BASE)/doc
	@install -m 644 $(DOCS)/reference/html/*.html $(BASE)/doc/html

	@tar cfz $(BASE)-$(VERSION).tgz $(BASE)
	@rm -rf $(BASE)

clean:
	@rm -rf $(BASE)-$(VERSION).tgz $(BASE) $(MU_SHELL) $(MU_SERVER)
