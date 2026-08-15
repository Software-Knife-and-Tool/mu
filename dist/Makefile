#
# dist makefile
#
.PHONY: dist clean release

VERSION := $(shell cargo pkgid | sed 's/.*\///' | sed 's/#/-/')

# mu
# ├── bin
# ├── doc
# │   └── html
# ├── lib
# │   ├── core
# │   ├── describe
# │   ├── fasl
# │   ├── format
# │   ├── image
# │   └── mu-repl
# ├── modules
# │   ├── common
# │   │   ├── describe
# │   │   └── metrics
# │   ├── deftype
# │   └── prelude
# │       └── repl
# └── system

PACKAGE = mu
RELEASE = ../target/release

DIST = ../dist
DOCS = ../doc
SRC = ../src
LIB = ../src/lib
UTILS = ../system

BIN = mu-sys mu-exec mu-server mu-repl mu-dev

# SYS = deftype.sys core.sys fasl.sys format.sys image.sys module.sys

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

	@install -d $(PACKAGE) $(PACKAGE)/bin $(PACKAGE)/doc $(PACKAGE)/lib $(PACKAGE)/modules $(PACKAGE)/system

	@for bin in $(BIN); do					\
	    install -m 755 $(RELEASE)/$$bin $(PACKAGE)/bin;	\
	done

	@for sys in $(SYSTEM); do				\
	    install -m 644 $(LIB)/dist/$$sys $(PACKAGE)/system;	\
	done

	@install -d $(PACKAGE)/doc/html
	@install -m 644 $(DOCS)/refcards/*.pdf $(PACKAGE)/doc
	@install -m 644 $(DOCS)/refcards/*.docx $(PACKAGE)/doc
	@install -m 644 $(DOCS)/reference/html/*.html $(PACKAGE)/doc/html

	@install -D -m 644 -t $(PACKAGE)/lib/core $(LIB)/sys/core/*.l $(LIB)/sys/core/*.def
	@install -D -m 644 -t $(PACKAGE)/lib/describe $(LIB)/sys/describe/*.l $(LIB)/sys/describe/*.def
	@install -D -m 644 -t $(PACKAGE)/lib/format $(LIB)/sys/format/*.l $(LIB)/sys/format/*.def
	@install -D -m 644 -t $(PACKAGE)/lib/fasl $(LIB)/sys/fasl/*.l $(LIB)/sys/fasl/*.def
	@install -D -m 644 -t $(PACKAGE)/lib/image $(LIB)/sys/image/*.l $(LIB)/sys/image/*.def
	@install -D -m 644 -t $(PACKAGE)/lib/mu-repl $(UTILS)/mu-repl/lib/mu-replrc

	@for module in $(MODULES); do	\
	    install -D -m 644 -t $(PACKAGE)/modules/$$module $(LIB)/modules/$$module/*.l;	\
	    install -D -m 644 -t $(PACKAGE)/modules/$$module $(LIB)/modules/$$module/mod.def;	\
	done

	@$(RELEASE)/mu-sys -l $(LIB)/dist/system.l -q '(system:require "../src/lib/sys/" "core")'
	@install -m 644 ./core.sys $(PACKAGE)/lib

	@$(RELEASE)/mu-sys -l $(LIB)/dist/system.l -l ./core.sys -q '(system:require "../src/lib/sys/" "describe")'
	@install -m 644 ./describe.sys $(PACKAGE)/lib

	@$(RELEASE)/mu-sys -l $(LIB)/dist/system.l -l ./core.sys -q '(system:require "../src/lib/sys/" "format")'
	@install -m 644 ./format.sys $(PACKAGE)/lib

	@$(RELEASE)/mu-sys -l $(LIB)/dist/system.l -l ./core.sys -q '(system:require "../src/lib/sys/" "module")'
	@install -m 644 ./module.sys $(PACKAGE)/lib

	@$(RELEASE)/mu-sys -l $(LIB)/dist/system.l -l ./core.sys -q '(system:require "../src/lib/modules/" "deftype")'
	@install -m 644 ./deftype.sys $(PACKAGE)/lib

#	@$(RELEASE)/mu-sys -l $(LIB)/dist/image.l -l ./core.sys -q '(system:require "../src/lib/sys/" "image")'
#	@install -m 644 ./image.sys $(PACKAGE)/lib
#	@$(RELEASE)/mu-sys -l $(LIB)/dist/fasl.l -l ./core.sys -q '(system:require "../src/lib/sys/" "fasl")'
#	@install -m 644 ./fasl.sys $(PACKAGE)/lib

	@tar --owner=root --group=root -czf $(VERSION).tgz ./$(PACKAGE)
	@rm -rf $(PACKAGE) $(BIN)

clean:
	@rm -rf $(VERSION).tgz $(PACKAGE) core.sys format.sys module.sys $(BIN)
