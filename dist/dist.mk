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
	load.l         \
	package.l      \
	vector.l 

COMMON= \
	common.l       \
	boole.l	       \
	defun.l        \
	describe.l     \
	predicates.l   \
	string.l       \
	fixnum.l       \
	list.l	       \
	print.l        \
	sequence.l     \
	stream.l       \
	symbol.l       \
	time.l

PRELUDE = \
	prelude.l      \
	break.l	       \
	loader.l       \
	sequence.l

dist:
	@rm -f core.l
	@for core in $(CORE); do			\
	    cat $(SRC)/core/$$core >> core.l;		\
	done
	@cp core.l mu/dist

	@rm -f common.l
	@for common in $(COMMON); do			\
	    cat $(SRC)/common/$$common >> common.l;	\
	done
	@cp common.l mu/dist

	@rm -f prelude.l
	@for prelude in $(PRELUDE); do			\
	    cat $(SRC)/prelude/$$prelude >> prelude.l;	\
	done
	@cp prelude.l mu/dist
