#
# valgrind makefile
#
VALGRIND='((:lambda (g) (crux:apply g (crux:cons g (crux:cons 101010101 (crux:cons 11011 ()))))) (:lambda (g a b) (:if (crux:eq 0 b) a (crux:apply g (crux:cons g (crux:cons b (crux:cons (crux:difference a (crux:product b (crux:quotient a b))) ()))))))) '

MU_SYS = ../../dist/mu-sys

help:
	@echo "valgrind makefile -----------------"
	@echo
	@echo "--- options"
	@echo "    valgrind - generate memcheck, callgrind, cachegrind reports"
	@echo "    clean - remove reports"

valgrind: memcheck callgrind cachegrind clean

memcheck:
	@valgrind --tool=memcheck --leak-check=yes --show-reachable=yes --num-callers=10 \
		--leak-resolution=med --show-leak-kinds=definite	\
		$(MU_SYS) -p      					\
		    -q $(VALGRIND) > memcheck.report 2>&1 || true

callgrind:
	@valgrind --tool=callgrind     	\
		$(MU_SYS) -p      	\
		    -q $(VALGRIND) > callgrind.report 2>&1 || true
	@callgrind_annotate --auto=yes callgrind.out.* >> callgrind.report

cachegrind:
	@valgrind --tool=cachegrind	\
		$(MU_SYS) -p      	\
		    -q $(VALGRIND) > cachegrind.report 2>&1 || true

clean:
	@rm -f *.out.*
