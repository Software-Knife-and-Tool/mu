#
# valgrind makefile
#
PHONY: clean
VALGRIND='((:lambda (g) (mu:apply g (mu:cons g (mu:cons 101010101 (mu:cons 11011 ()))))) (:lambda (g a b) (:if (mu:eq 0 b) a (mu:apply g (mu:cons g (mu:cons b (mu:cons (mu:sub a (mu:mul b (mu:div a b))) ()))))))) '

MU_SYS = ../../dist/mu-sys

help:
	@echo "valgrind makefile -----------------"
	@echo
	@echo "--- options"
	@echo "    valgrind - generate memcheck, callgrind, cachegrind reports for test form"
	@echo "    null - generate memcheck, callgrind, cachegrind reports for no form"
	@echo "    all - valgrind null"
	@echo "    clean - remove reports"

all: valgrind null
valgrind: memcheck callgrind cachegrind
null: nullcheck nullcall nullcache

memcheck:
	@valgrind --tool=memcheck --leak-check=yes --show-reachable=yes --num-callers=10 \
		--leak-resolution=med --show-leak-kinds=definite	\
		$(MU_SYS)	      					\
		    -q $(VALGRIND) > memcheck.report 2>&1 || true

callgrind:
	@valgrind --tool=callgrind     	\
		$(MU_SYS)	      	\
		    -q $(VALGRIND) > callgrind.report 2>&1 || true
	@callgrind_annotate --auto=yes callgrind.out.* >> callgrind.report

cachegrind:
	@valgrind --tool=cachegrind	\
		$(MU_SYS)	      	\
		    -q $(VALGRIND) > cachegrind.report 2>&1 || true
	@make clean

nullcheck:
	@valgrind --tool=memcheck --leak-check=yes --show-reachable=yes --num-callers=10 \
		--leak-resolution=med --show-leak-kinds=definite	\
		$(MU_SYS) > nullcheck.report 2>&1 || true

nullcall:
	@valgrind --tool=callgrind     	\
		$(MU_SYS) > nullcall.report 2>&1 || true
	@callgrind_annotate --auto=yes callgrind.out.* >> nullcall.report

nullcache:
	@valgrind --tool=cachegrind	\
		$(MU_SYS) > nullcache.report 2>&1 || true
	@make clean

clean:
	@rm -f *.out.*
