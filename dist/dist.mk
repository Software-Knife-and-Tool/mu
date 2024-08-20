#
# dist makefile
#
.PHONY: mu prelude
SRC = ../src

# core.l needs to be first
CORE= \
	core.l         \
	compile.l      \
	environment.l  \
	string.l       \
	typedef.l      \
	typespec.l     \
	exception.l    \
	fixnum.l       \
	format.l       \
	funcall.l      \
	function.l     \
	lambda.l       \
	list.l         \
	macro.l        \
	map.l          \
	namespace.l    \
	parse.l        \
	quasiquote.l   \
	read.l         \
	read-macro.l   \
	stream.l       \
	symbol.l       \
	symbol-macro.l \
	typespec.l     \
	vector.l

REPL = \
	break.l	       \
	repl.l

INSPECT = \
	describe.l     \
	inspect.l

SYSTEM = \
	loader.l       \
	log.l          \
	time.l

dist:
	@cp -r $(SRC)/core mu/lib
	@cp -r $(SRC)/lib/codegen mu/lib
	@cp -r $(SRC)/lib/common mu/lib
	@cp -r $(SRC)/lib/mu mu/lib
	@cp -r $(SRC)/prelude mu/lib

	@rm -rf core
	@mkdir core
	@for core in $(CORE); do				\
	    cat $(SRC)/core/$$core >> core.l;			\
	done
	@cp core.l mu/lib/core
	@mv core.l core

	@for repl in $(REPL); do				\
	    cat $(SRC)/prelude/repl/$$repl >> repl.l;		\
	done
	@cp repl.l mu/lib/prelude
	@mv repl.l prelude

	@for inspect in $(INSPECT); do				\
	    cat $(SRC)/prelude/inspect/$$inspect >> inspect.l;	\
	done
	@cp inspect.l mu/lib/prelude
	@mv inspect.l prelude
	@for system in $(SYSTEM); do				\
	    cat $(SRC)/prelude/system/$$system >> system.l;	\
	done
	@cp system.l mu/lib/prelude
	@mv system.l prelude
