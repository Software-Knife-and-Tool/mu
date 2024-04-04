#
# mu makefile
#
.PHONY: mu prelude
SRC = ../src

# prelude.l needs to be first
PRELUDE = \
	prelude.l	\
	break.l		\
	compile.l	\
	ctype.l		\
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

mu:
	@cp -r $(SRC)/codegen mu/lib
	@cp -r $(SRC)/mu mu/lib
	@cp -r $(SRC)/prelude mu/lib
	@rm -f prelude.l
	@for prelude in $(PRELUDE); do		\
	    cat $(SRC)/prelude/$$prelude >> prelude.l;	\
	done
	@cp prelude.l mu/lib
