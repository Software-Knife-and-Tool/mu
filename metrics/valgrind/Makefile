#
# valgrind makefile
#
VALGRIND='((:lambda (g) (mu:apply g (mu:cons g (mu:cons 101010101 (mu:cons 11011 ()))))) (:lambda (g a b) (:if (mu:eq 0 b) a (mu:apply g (mu:cons g (mu:cons b (mu:cons (mu:fx-sub a (mu:fx-mul b (mu:fx-div a b))) ()))))))) '

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
		--leak-resolution=med --show-leak-kinds=definite \
		$(MU_SYS) -p      \
		    -q $(VALGRIND) > memcheck.report 2>&1 || true

callgrind:
	@valgrind --tool=callgrind     	\
		$(MU_SYS) -p      \
		    -q $(VALGRIND) > callgrind.report 2>&1 || true
	@callgrind_annotate --auto=yes callgrind.out.* >> callgrind.report

cachegrind:
	@valgrind --tool=cachegrind	\
		$(MU_SYS) -p      \
		    -q $(VALGRIND) > cachegrind.report 2>&1 || true

clean:
	@rm -f *.out.*
