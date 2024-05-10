;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol macros
;;;

;;;
;;; symbol readers
;;;
(core:intern prelude "%read-keywordp"
  (:lambda (name)
      (core:eq #\: (prelude:schar name 0))))

(core:intern prelude "%read-symbol-scope"
  (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             :extern
             (:if (core:eq #\: (prelude:schar name (core:sum 1 colon)))
                  :intern
                  :extern)))
        (prelude:string-position #\: name))))

(core:intern prelude "%read-symbol-ns"
  (:lambda (name)
    ((:lambda (colon)
       (:if (prelude:null colon)
            ()
            (core:find-ns (prelude:substr name 0 (core:difference colon 1)))))
     (prelude:string-position #\: name))))

(core:intern prelude "%read-symbol-name"
   (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             name
             (prelude:substr name
                (:if (core:eq #\: (prelude:schar name (core:sum 1 colon)))
                     (core:sum 2 colon)
                     (core:sum 1 colon))
                (:if (core:eq #\: (prelude:schar name (core:sum 1 colon)))
                     (core:sum colon (core:difference (core:vector-len name) colon))
                     (core:sum colon (core:difference (core:vector-len name) (core:sum 1 colon)))))))
      (prelude:string-position #\: name))))

(core:intern prelude "%read-symbol"
   (:lambda (symbol)
     (:if (prelude:%read-keywordp symbol)
          (prelude:make-keyword (prelude:%read-symbol-name symbol))
          ((:lambda (ns name)
             (:if (prelude:null ns)
                  ((:lambda (symbol)
                     (:if (prelude:null symbol)
                          (core:makunbound (core:intern (core:find-ns "") name ()))
                          symbol))
                   (core:find (core:find-ns "") name))
                  ((:lambda (symbol)
                     (:if (prelude:null symbol)
                          (core:makunbound (core:intern ns name ()))
                          symbol))
                   (core:find ns name))))
             (prelude:%read-symbol-ns symbol)
             (prelude:%read-symbol-name symbol)))))

;;;
;;; symbol macros
;;;
(core:intern prelude "%symbol-macro-expand"
   (:lambda (symbol)
     (:if (prelude:%orf (prelude:null symbol) (prelude:keywordp symbol))
          symbol
          (:if (prelude:uninternedp symbol)
               ((:lambda (sym)
                  (:if sym
                       (core:eval (prelude:symbol-value sym))
                       symbol))
                (prelude:%get-symbol-macro symbol))
               symbol))))

(core:intern prelude "%get-symbol-macro"
   (:lambda (symbol)
      (:if (core:eq :symbol (core:type-of symbol))
           (core:find prelude:%symbol-macro-ns% (prelude:symbol-name symbol))
           (prelude:raise symbol 'prelude:%get-symbol-macro "not a symbol"))))

(core:intern prelude "define-symbol-macro"
   (:lambda (symbol form)
      (:if (core:eq :symbol (core:type-of symbol))
           (core:intern prelude:%symbol-macro-ns% (core:symbol-name symbol) form)
           (prelude:raise symbol 'prelude:define-symbol-macro "not a symbol"))))

;;; (prelude:define-symbol-macro 't :t)
;;; (prelude:define-symbol-macro 'nil :nil)
