#
# dist makefile
#
.PHONY: mu prelude
SRC = ../src
LIB = $(SRC)/lib

# prelude.l needs to be first
PRELUDE = \
	prelude.l	\
	break.l		\
	compile.l	\
	describe.l	\
	environment.l	\
	exception.l	\
	fixnum.l	\
	format.l	\
	funcall.l	\
	function.l	\
	inspect.l	\
	lambda.l	\
	list.l		\
	loader.l	\
	log.l		\
	macro.l		\
	map.l		\
	namespace.l	\
	parse.l		\
	quasiquote.l	\
	read-macro.l	\
	read.l		\
	repl.l		\
	stream.l	\
	string.l	\
	symbol-macro.l	\
	symbol.l	\
	time.l		\
	type.l		\
	vector.l

dist:
	@cp -r $(LIB)/codegen mu/lib
	@cp -r $(LIB)/common mu/lib
	@cp -r $(LIB)/mu mu/lib
	@cp -r $(LIB)/prelude mu/lib
	@rm -f prelude.l
	@for prelude in $(PRELUDE); do			\
	    cat $(LIB)/prelude/$$prelude >> prelude.l;	\
	done
	@cp prelude.l mu/lib
