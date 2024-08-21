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

PRELUDE = \
	prelude.l

dist:
	@rm -f core.l
	@for core in $(CORE); do			\
	    cat $(SRC)/core/$$core >> core.l;		\
	done
	@cp core.l mu/dist

	@rm -f prelude.l
	@for prelude in $(PRELUDE); do			\
	    cat $(SRC)/prelude/$$prelude >> prelude.l;	\
	done
	@cp prelude.l mu/dist
