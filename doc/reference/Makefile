#
# docs makefile
#
.PHONY: reference clobber

# SRCDIRS = foreward core mu mu preface system
SRCDIRS = foreward prelude

# page break <div style="page-break-after: always"></div>
BOOK= foreward/index.md prelude/2-1.prelude.md prelude/2-2.reader.md prelude/2-3.compile.md prelude/2-4.lambda.md prelude/2-5.functions.md prelude/2-6.macros.md prelude/2-7.sequences.md prelude/2-8.exceptions.md prelude/2-9.lists.md prelude/2-10.streams.md prelude/2-11.utilities.md prelude/2-12.prelude-symbols.md

reference:
	@for dir in $(SRCDIRS); do		\
	    for md in $$dir/*.md; do		\
	    	pandoc -s --css=./reference.css $$md -f markdown -o ./html/`basename $$md .md`.html;	\
	    done 				\
	done

clobber:
	@rm -f html/*.html
