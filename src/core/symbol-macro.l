;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol macros
;;;

;;;
;;; symbol readers
;;;
(mu:intern core "%read-keywordp"
  (:lambda (name)
      (mu:eq #\: (core:schar name 0))))

(mu:intern core "%read-symbol-scope"
  (:lambda (name)
     ((:lambda (colon)
        (:if (core:null colon)
             :extern
             (:if (mu:eq #\: (core:schar name (mu:sum 1 colon)))
                  :intern
                  :extern)))
        (core:string-position #\: name))))

(mu:intern core "%read-symbol-ns"
  (:lambda (name)
    ((:lambda (colon)
       (:if (core:null colon)
            ()
            (mu:find-namespace (core:substr name 0 (mu:difference colon 1)))))
     (core:string-position #\: name))))

(mu:intern core "%read-symbol-name"
   (:lambda (name)
     ((:lambda (colon)
        (:if (core:null colon)
             name
             (core:substr name
                (:if (mu:eq #\: (core:schar name (mu:sum 1 colon)))
                     (mu:sum 2 colon)
                     (mu:sum 1 colon))
                (:if (mu:eq #\: (core:schar name (mu:sum 1 colon)))
                     (mu:sum colon (mu:difference (mu:vector-len name) colon))
                     (mu:sum colon (mu:difference (mu:vector-len name) (mu:sum 1 colon)))))))
      (core:string-position #\: name))))

(mu:intern core "%read-symbol"
   (:lambda (symbol)
     (:if (core:%read-keywordp symbol)
          (core:make-keyword (core:%read-symbol-name symbol))
          ((:lambda (ns name)
             (:if (core:null ns)
                  ((:lambda (symbol)
                     (:if (core:null symbol)
                          (mu:makunbound (mu:intern (mu:find-namespace "") name ()))
                          symbol))
                   (mu:find (mu:find-namespace "") name))
                  ((:lambda (symbol)
                     (:if (core:null symbol)
                          (mu:makunbound (mu:intern ns name ()))
                          symbol))
                   (mu:find ns name))))
             (core:%read-symbol-ns symbol)
             (core:%read-symbol-name symbol)))))

;;;
;;; symbol macros
;;;
(mu:intern core "%symbol-macro-expand"
   (:lambda (symbol)
     (:if (core:%orf (core:null symbol) (core:keywordp symbol))
          symbol
          (:if (core:uninternedp symbol)
               ((:lambda (sym)
                  (:if sym
                       (mu:eval (core:symbol-value sym))
                       symbol))
                (core:%get-symbol-macro symbol))
               symbol))))

(mu:intern core "%get-symbol-macro"
   (:lambda (symbol)
      (:if (mu:eq :symbol (mu:type-of symbol))
           (mu:find core:%symbol-macro-ns% (core:symbol-name symbol))
           (core:raise symbol 'core:%get-symbol-macro "not a symbol"))))

(mu:intern core "define-symbol-macro"
   (:lambda (symbol form)
      (:if (mu:eq :symbol (mu:type-of symbol))
           (mu:intern core:%symbol-macro-ns% (mu:symbol-name symbol) form)
           (core:raise symbol 'core:define-symbol-macro "not a symbol"))))

;;; (core:define-symbol-macro 't :t)
;;; (core:define-symbol-macro 'nil :nil)
