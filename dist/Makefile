#
# dist makefile
#
.PHONY: dist clean release

VERSION := $(shell cargo pkgid | sed 's/.*#//')

# mu
# ├── bin
# ├── doc
# │   └── html
# ├── lib
# │   ├── core
# │   ├── fasl
# │   ├── format
# │   ├── image
# ├── modules
# │   ├── common
# │   │   ├── describe
# │   │   └── metrics
# │   ├── deftype
# │   └── prelude
# │       └── repl
# └── system
#

RELEASE = ../target/release
MU = ./mu
DIST = ../dist
DOCS = ../doc
SRC = ../src
LIB = ../src/lib

BIN = mu-sys mu-exec mu-server sys-repl sys-dev

SYS = core.sys fasl.sys format.sys image.sys module.sys

SYSTEM = system.l image.l fasl.l

MODULES = 		\
	common		\
	common/describe	\
	common/metrics	\
	deftype		\
	prelude		\
	prelude/repl

dist:
	@echo $(VERSION) > VERSION

	@install -d $(MU) $(MU)/bin $(MU)/doc $(MU)/lib $(MU)/modules $(MU)/system

	@for bin in $(BIN); do				\
	    install -m 755 $(RELEASE)/$$bin $(MU)/bin;	\
	done

	@for sys in $(SYSTEM); do				\
	    install -m 644 $(LIB)/dist/$$sys $(MU)/system;	\
	done

	@install -d $(MU)/doc/html
	@install -m 644 $(DOCS)/refcards/*.pdf $(MU)/doc
	@install -m 644 $(DOCS)/refcards/*.docx $(MU)/doc
	@install -m 644 $(DOCS)/reference/html/*.html $(MU)/doc/html

	@install -D -m 644 -t $(MU)/lib/core $(LIB)/sys/core/*.l $(LIB)/sys/core/*.def
	@install -D -m 644 -t $(MU)/lib/format $(LIB)/sys/format/*.l $(LIB)/sys/format/*.def
	@install -D -m 644 -t $(MU)/lib/fasl $(LIB)/sys/fasl/*.l $(LIB)/sys/fasl/*.def
	@install -D -m 644 -t $(MU)/lib/image $(LIB)/sys/image/*.l $(LIB)/sys/image/*.def

	@for module in $(MODULES); do	\
	    install -D -m 644 -t $(MU)/modules/$$module $(LIB)/modules/$$module/*.l;	\
	    install -D -m 644 -t $(MU)/modules/$$module $(LIB)/modules/$$module/mod.def;	\
	done

	@$(RELEASE)/mu-sys -l $(LIB)/dist/system.l -q '(system:require "../src/lib/sys/" "core")'
	@install -m 644 ./core.sys $(MU)/lib

	@$(RELEASE)/mu-sys -l $(LIB)/dist/system.l -l ./core.sys -q '(system:require "../src/lib/sys/" "format")'
	@install -m 644 ./format.sys $(MU)/lib

	@$(RELEASE)/mu-sys -l $(LIB)/dist/system.l -l ./core.sys -q '(system:require "../src/lib/sys/" "module")'
	@install -m 644 ./module.sys $(MU)/lib

	@tar --owner=root --group=root -czf $(MU)-$(VERSION).tgz $(MU)
	@rm -rf $(MU) $(BIN)

clean:
	@rm -rf $(MU)-*.tgz $(MU) core.sys $(BIN)
