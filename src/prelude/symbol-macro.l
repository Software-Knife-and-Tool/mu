;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol macros
;;;

;;;
;;; symbol readers
;;;
(crux:intern prelude "%read-keywordp"
  (:lambda (name)
      (crux:eq #\: (prelude:schar name 0))))

(crux:intern prelude "%read-symbol-scope"
  (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             :extern
             (:if (crux:eq #\: (prelude:schar name (crux:sum 1 colon)))
                  :intern
                  :extern)))
        (prelude:string-position #\: name))))

(crux:intern prelude "%read-symbol-ns"
  (:lambda (name)
    ((:lambda (colon)
       (:if (prelude:null colon)
            ()
            (crux:find-ns (prelude:substr name 0 (crux:difference colon 1)))))
     (prelude:string-position #\: name))))

(crux:intern prelude "%read-symbol-name"
   (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             name
             (prelude:substr name
                (:if (crux:eq #\: (prelude:schar name (crux:sum 1 colon)))
                     (crux:sum 2 colon)
                     (crux:sum 1 colon))
                (:if (crux:eq #\: (prelude:schar name (crux:sum 1 colon)))
                     (crux:sum colon (crux:difference (crux:vector-len name) colon))
                     (crux:sum colon (crux:difference (crux:vector-len name) (crux:sum 1 colon)))))))
      (prelude:string-position #\: name))))

(crux:intern prelude "%read-symbol"
   (:lambda (symbol)
     (:if (prelude:%read-keywordp symbol)
          (prelude:make-keyword (prelude:%read-symbol-name symbol))
          ((:lambda (ns name)
             (:if (prelude:null ns)
                  ((:lambda (symbol)
                     (:if (prelude:null symbol)
                          (crux:makunbound (crux:intern (crux:find-ns "") name ()))
                          symbol))
                   (crux:find (crux:find-ns "") name))
                  ((:lambda (symbol)
                     (:if (prelude:null symbol)
                          (crux:makunbound (crux:intern ns name ()))
                          symbol))
                   (crux:find ns name))))
             (prelude:%read-symbol-ns symbol)
             (prelude:%read-symbol-name symbol)))))

;;;
;;; symbol macros
;;;
(crux:intern prelude "%symbol-macro-expand"
   (:lambda (symbol)
     (:if (prelude:%orf (prelude:null symbol) (prelude:keywordp symbol))
          symbol
          (:if (prelude:uninternedp symbol)
               ((:lambda (sym)
                  (:if sym
                       (crux:eval (prelude:symbol-value sym))
                       symbol))
                (prelude:%get-symbol-macro symbol))
               symbol))))

(crux:intern prelude "%get-symbol-macro"
   (:lambda (symbol)
      (:if (crux:eq :symbol (crux:type-of symbol))
           (crux:find prelude:%symbol-macro-ns% (prelude:symbol-name symbol))
           (prelude:raise symbol 'prelude:%get-symbol-macro "not a symbol"))))

(crux:intern prelude "define-symbol-macro"
   (:lambda (symbol form)
      (:if (crux:eq :symbol (crux:type-of symbol))
           (crux:intern prelude:%symbol-macro-ns% (crux:symbol-name symbol) form)
           (prelude:raise symbol 'prelude:define-symbol-macro "not a symbol"))))

;;; (prelude:define-symbol-macro 't :t)
;;; (prelude:define-symbol-macro 'nil :nil)
