#
# dist makefile
#
.PHONY: mu prelude
SRC = ../src

# core.l needs to be first
CORE= \
	core.l         \
	compile.l      \
	string.l       \
	fixnum.l       \
	format.l       \
	funcall.l      \
	list.l         \
	map.l          \
	stream.l       \
	macro.l        \
	read.l         \
	quasiquote.l   \
	parse.l        \
	read-macro.l   \
	read-macro.l   \
	symbol.l       \
	symbol-macro.l \
	deftype.l      \
	typespec.l     \
	exception.l    \
	closure.l      \
	lambda.l       \
	package.l      \
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
