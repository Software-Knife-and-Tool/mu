#
# dist makefile
#
.PHONY: mu prelude
SRC = ../src
LIB = $(SRC)/lib

# prelude.l needs to be first
CORE= \
	prelude.l	\
	compile.l	\
	environment.l	\
	exception.l	\
	fixnum.l	\
	format.l	\
	funcall.l	\
	function.l	\
	lambda.l	\
	list.l		\
	macro.l		\
	map.l		\
	namespace.l	\
	parse.l		\
	quasiquote.l	\
	read-macro.l	\
	read.l		\
	stream.l	\
	string.l	\
	symbol-macro.l	\
	symbol.l	\
	type.l		\
	vector.l

REPL = \
	break.l		\
	repl.l

INSPECT = \
	describe.l	\
	inspect.l

SYSTEM = \
	loader.l	\
	log.l		\
	time.l

dist:
	@cp -r $(LIB)/codegen mu/lib
	@cp -r $(LIB)/common mu/lib
	@cp -r $(LIB)/mu mu/lib
	@cp -r $(LIB)/prelude mu/lib
	@rm -f prelude/*.l
	@for core in $(CORE); do				\
	    cat $(LIB)/prelude/core/$$core >> prelude.l;	\
	done
	@cp prelude.l mu/lib/prelude
	@mv prelude.l prelude
	@for repl in $(REPL); do				\
	    cat $(LIB)/prelude/repl/$$repl >> repl.l;		\
	done
	@cp repl.l mu/lib/prelude
	@mv repl.l prelude
	@for inspect in $(INSPECT); do				\
	    cat $(LIB)/prelude/inspect/$$inspect >> inspect.l;	\
	done
	@cp inspect.l mu/lib/prelude
	@mv inspect.l prelude
	@for system in $(SYSTEM); do				\
	    cat $(LIB)/prelude/system/$$system >> system.l;	\
	done
	@cp system.l mu/lib/prelude
	@mv system.l prelude
