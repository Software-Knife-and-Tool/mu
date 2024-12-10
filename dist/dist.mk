#
# dist makefile
#
.PHONY: mu prelude
SRC = ../src

CORE= \
	testing.l	\
	base.l		\
	compile.l	\
	string.l	\
	format.l	\
	funcall.l	\
	load.l		\
	module.l	\
	list.l		\
	map.l		\
	stream.l	\
	read.l		\
	quasi.l		\
	parse.l		\
	read-macro.l	\
	symbol.l	\
	symbol-macro.l	\
	deftype.l	\
	typespec.l	\
	exception.l	\
	macro.l		\
	closure.l	\
	vector.l	\
	common.l

COMMON= \
	testing.l      \
	boole.l	       \
	defun.l        \
	describe.l     \
	fixnum.l       \
	list.l	       \
	package.l      \
	predicates.l   \
	print.l        \
	sequence.l     \
	stream.l       \
	string.l       \
	symbol.l       \
	time.l

PRELUDE = \
	testing.l      \
	list.l	       \
	loader.l

dist:
	@rm -f core.l
	@for core in $(CORE); do			\
	    cat $(SRC)/core/$$core >> core.l;		\
	done
	@cp core.l mu/dist

	@rm -f common.l
	@for common in $(COMMON); do			\
	    cat $(SRC)/modules/common/$$common >> common.l;	\
	done

	@rm -f prelude.l
	@for prelude in $(PRELUDE); do			\
	    cat $(SRC)/modules/prelude/$$prelude >> prelude.l;	\
	done
	@cp prelude.l mu/dist
