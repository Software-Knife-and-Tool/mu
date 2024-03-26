;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; macro definition, compilation, and expansion
;;;

;;;
;;; Macro definitions are compiled like lambdas (though eventually with a slighlty
;;; different lambda list including &whole).
;;;
;;; Macro calls are expanded at compile time.
;;;
;;; Common Lisp adds an environment argument here for lexical macro expanders.
;;;

;;;
;;; macro environment
;;;
(libcore:intern :prelude "%map-macro-symbol-ns"
   (:lambda (symbol)
      ((:lambda (ns-name)
          (libcore:ns-find prelude:%macro-ns% ns-name))
       (libcore:sy-name (libcore:sy-ns symbol)))))

(libcore:intern :prelude "%make-macro-ns"
   (:lambda (ns)
      ((:lambda (macro-ns)
          (libcore:intern prelude:%macro-ns% (libcore:sy-name ns) macro-ns)
          macro-ns)
       (prelude:%gen-anon-ns))))

(libcore:intern :prelude "%defmacro"
   (:lambda (symbol macro-function)
      ((:lambda (macro-ns symbol-ns)
         (:if macro-ns
              (libcore:intern macro-ns symbol-ns macro-function)
              (libcore:intern (prelude:%make-macro-ns symbol-ns) (libcore:sy-name symbol) macro-function))
         symbol)
       (prelude:%map-macro-symbol-ns symbol)
       (libcore:sy-ns symbol))))

(libcore:intern :prelude "%find-macro"
   (:lambda (symbol env)
      ((:lambda (macro-ns symbol-ns symbol-name)
          (:if macro-ns
               (libcore:ns-find (libcore:sy-val macro-ns) symbol-name)
               ()))
       (libcore:ns-find
        prelude:%macro-ns%
        (libcore:sy-name (libcore:sy-ns symbol)))
       (libcore:sy-name (libcore:sy-ns symbol))
       (libcore:sy-name symbol))))

(libcore:intern :prelude "%compile-macro-call"
   (:lambda (macro-symbol arg-list env)
       ((:lambda (macro-function)
          ((:lambda (expanded-form)
             (prelude:%compile expanded-form env))
           (:if (prelude:%prelude-function-p macro-function)
                (prelude:%funct-apply-quoted
                 macro-function
                 (libcore:eval (prelude:%compile-lambda-arg-list macro-function arg-list env)))
                (libcore:apply macro-function arg-list))))
      (prelude:macro-function macro-symbol env))))

;;;
;;; macroexpand-1 expands a macro call once.
;;;
(libcore:intern :prelude "macroexpand-1"
   (:lambda (form env)
     (:if (prelude:consp form)
          ((:lambda (fn-symbol arg-list)
             (:if (libcore:eq :symbol (libcore:type-of fn-symbol))
                  ((:lambda (macro-function)
                     (:if macro-function
                          (:if (prelude:%prelude-function-p macro-function)
                               (prelude:%funct-apply-quoted macro-function (prelude:%quoted-lambda-arg-list macro-function arg-list))
                               (libcore:apply macro-function arg-list))
                          form))
                   (prelude:macro-function fn-symbol env))
                  form))
           (libcore:car form)
           (libcore:cdr form))
          form)))

;;;
;;; macroexpand: expand a form until it is no longer a macro call
;;;
(libcore:intern :prelude "macroexpand"
   (:lambda (form env)
     ((:lambda (expanded)
          (:if (libcore:eq form expanded)
             expanded
             (prelude:macroexpand expanded env)))
       (prelude:macroexpand-1 form env))))

;;;
;;; macro-function: map a symbol to a macro function
;;;
;;; acts as a predicate
;;;
(libcore:intern :prelude "macro-function"
   (:lambda (symbol env)
      (:if (libcore:eq :symbol (libcore:type-of symbol))
           ((:lambda (macro)
               (:if macro
                    (libcore:sy-val macro)
                    ()))
            (prelude:%find-macro symbol env))
          ())))
