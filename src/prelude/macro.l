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
(crux:intern prelude "%map-macro-symbol-ns"
   (:lambda (symbol)
      ((:lambda (ns)
          (crux:find prelude:%macro-ns% (crux:ns-name ns)))
       (crux:symbol-ns symbol))))

(crux:intern prelude "%make-macro-ns"
   (:lambda (ns)
       ((:lambda (macro-ns)
          (crux:intern prelude:%macro-ns% (crux:ns-name ns) macro-ns)
          macro-ns)
       (prelude:%gen-anon-ns))))

(crux:intern prelude "%defmacro"
   (:lambda (symbol macro-function)
      ((:lambda (macro-ns symbol-ns)
         (:if macro-ns
              (crux:intern macro-ns symbol-ns macro-function)
              (crux:intern (prelude:%make-macro-ns symbol-ns) (crux:symbol-name symbol) macro-function))
         symbol)
       (prelude:%map-macro-symbol-ns symbol)
       (crux:symbol-ns symbol))))

(crux:intern prelude "%find-macro"
    (:lambda (symbol env)
      ((:lambda (macro-ns symbol-name)
          (:if macro-ns
               (crux:find (crux:symbol-value macro-ns) symbol-name)
               ()))
       (crux:find prelude:%macro-ns% (crux:ns-name (crux:symbol-ns symbol)))
       (crux:symbol-name symbol))))

(crux:intern prelude "%compile-macro-call"
   (:lambda (macro-symbol arg-list env)
       ((:lambda (macro-function)
          ((:lambda (expanded-form)
             (prelude:%compile expanded-form env))
           (:if (prelude:%prelude-function-p macro-function)
                (prelude:%fn-apply-quoted
                 macro-function
                 (crux:eval (prelude:%compile-lambda-arg-list macro-function arg-list env)))
                (crux:apply macro-function arg-list))))
      (prelude:macro-function macro-symbol env))))

;;;
;;; macroexpand-1 expands a macro call once.
;;;
(crux:intern prelude "macroexpand-1"
   (:lambda (form env)
     (:if (prelude:consp form)
          ((:lambda (fn-symbol arg-list)
             (:if (crux:eq :symbol (crux:type-of fn-symbol))
                  ((:lambda (macro-function)
                     (:if macro-function
                          (:if (prelude:%prelude-function-p macro-function)
                               (prelude:%fn-apply-quoted macro-function (prelude:%quoted-lambda-arg-list macro-function arg-list))
                               (crux:apply macro-function arg-list))
                          form))
                   (prelude:macro-function fn-symbol env))
                  form))
           (crux:car form)
           (crux:cdr form))
          form)))

;;;
;;; macroexpand: expand a form until it is no longer a macro call
;;;
(crux:intern prelude "macroexpand"
   (:lambda (form env)
     ((:lambda (expanded)
          (:if (crux:eq form expanded)
             expanded
             (prelude:macroexpand expanded env)))
       (prelude:macroexpand-1 form env))))

;;;
;;; macro-function: map a symbol to a macro function
;;;
;;; acts as a predicate
;;;
(crux:intern prelude "macro-function"
   (:lambda (symbol env)
      (:if (crux:eq :symbol (crux:type-of symbol))
           ((:lambda (macro)
               (:if macro
                    (crux:symbol-value macro)
                    ()))
            (prelude:%find-macro symbol env))
          ())))
