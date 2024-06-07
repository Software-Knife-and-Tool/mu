;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol macros
;;;

;;;
;;; symbol readers
;;;
(mu:intern prelude "%read-keywordp"
  (:lambda (name)
      (mu:eq #\: (prelude:schar name 0))))

(mu:intern prelude "%read-symbol-scope"
  (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             :extern
             (:if (mu:eq #\: (prelude:schar name (mu:sum 1 colon)))
                  :intern
                  :extern)))
        (prelude:string-position #\: name))))

(mu:intern prelude "%read-symbol-ns"
  (:lambda (name)
    ((:lambda (colon)
       (:if (prelude:null colon)
            ()
            (mu:find-ns (prelude:substr name 0 (mu:difference colon 1)))))
     (prelude:string-position #\: name))))

(mu:intern prelude "%read-symbol-name"
   (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             name
             (prelude:substr name
                (:if (mu:eq #\: (prelude:schar name (mu:sum 1 colon)))
                     (mu:sum 2 colon)
                     (mu:sum 1 colon))
                (:if (mu:eq #\: (prelude:schar name (mu:sum 1 colon)))
                     (mu:sum colon (mu:difference (mu:vector-len name) colon))
                     (mu:sum colon (mu:difference (mu:vector-len name) (mu:sum 1 colon)))))))
      (prelude:string-position #\: name))))

(mu:intern prelude "%read-symbol"
   (:lambda (symbol)
     (:if (prelude:%read-keywordp symbol)
          (prelude:make-keyword (prelude:%read-symbol-name symbol))
          ((:lambda (ns name)
             (:if (prelude:null ns)
                  ((:lambda (symbol)
                     (:if (prelude:null symbol)
                          (mu:makunbound (mu:intern (mu:find-ns "") name ()))
                          symbol))
                   (mu:find (mu:find-ns "") name))
                  ((:lambda (symbol)
                     (:if (prelude:null symbol)
                          (mu:makunbound (mu:intern ns name ()))
                          symbol))
                   (mu:find ns name))))
             (prelude:%read-symbol-ns symbol)
             (prelude:%read-symbol-name symbol)))))

;;;
;;; symbol macros
;;;
(mu:intern prelude "%symbol-macro-expand"
   (:lambda (symbol)
     (:if (prelude:%orf (prelude:null symbol) (prelude:keywordp symbol))
          symbol
          (:if (prelude:uninternedp symbol)
               ((:lambda (sym)
                  (:if sym
                       (mu:eval (prelude:symbol-value sym))
                       symbol))
                (prelude:%get-symbol-macro symbol))
               symbol))))

(mu:intern prelude "%get-symbol-macro"
   (:lambda (symbol)
      (:if (mu:eq :symbol (mu:type-of symbol))
           (mu:find prelude:%symbol-macro-ns% (prelude:symbol-name symbol))
           (prelude:raise symbol 'prelude:%get-symbol-macro "not a symbol"))))

(mu:intern prelude "define-symbol-macro"
   (:lambda (symbol form)
      (:if (mu:eq :symbol (mu:type-of symbol))
           (mu:intern prelude:%symbol-macro-ns% (mu:symbol-name symbol) form)
           (prelude:raise symbol 'prelude:define-symbol-macro "not a symbol"))))

;;; (prelude:define-symbol-macro 't :t)
;;; (prelude:define-symbol-macro 'nil :nil)
