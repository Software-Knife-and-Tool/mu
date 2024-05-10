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
(core:intern prelude "%map-macro-symbol-ns"
   (:lambda (symbol)
      ((:lambda (ns)
          (core:find prelude:%macro-ns% (core:ns-name ns)))
       (core:symbol-ns symbol))))

(core:intern prelude "%make-macro-ns"
   (:lambda (ns)
       ((:lambda (macro-ns)
          (core:intern prelude:%macro-ns% (core:ns-name ns) macro-ns)
          macro-ns)
       (prelude:%gen-anon-ns))))

(core:intern prelude "%defmacro"
   (:lambda (symbol macro-function)
      ((:lambda (macro-ns symbol-ns)
         (:if macro-ns
              (core:intern macro-ns symbol-ns macro-function)
              (core:intern (prelude:%make-macro-ns symbol-ns) (core:symbol-name symbol) macro-function))
         symbol)
       (prelude:%map-macro-symbol-ns symbol)
       (core:symbol-ns symbol))))

(core:intern prelude "%find-macro"
    (:lambda (symbol env)
      ((:lambda (macro-ns symbol-name)
          (:if macro-ns
               (core:find (core:symbol-value macro-ns) symbol-name)
               ()))
       (core:find prelude:%macro-ns% (core:ns-name (core:symbol-ns symbol)))
       (core:symbol-name symbol))))

(core:intern prelude "%compile-macro-call"
   (:lambda (macro-symbol arg-list env)
       ((:lambda (macro-function)
          ((:lambda (expanded-form)
             (prelude:%compile expanded-form env))
           (:if (prelude:%prelude-function-p macro-function)
                (prelude:%fn-apply-quoted
                 macro-function
                 (core:eval (prelude:%compile-lambda-arg-list macro-function arg-list env)))
                (core:apply macro-function arg-list))))
      (prelude:macro-function macro-symbol env))))

;;;
;;; macroexpand-1 expands a macro call once.
;;;
(core:intern prelude "macroexpand-1"
   (:lambda (form env)
     (:if (prelude:consp form)
          ((:lambda (fn-symbol arg-list)
             (:if (core:eq :symbol (core:type-of fn-symbol))
                  ((:lambda (macro-function)
                     (:if macro-function
                          (:if (prelude:%prelude-function-p macro-function)
                               (prelude:%fn-apply-quoted macro-function (prelude:%quoted-lambda-arg-list macro-function arg-list))
                               (core:apply macro-function arg-list))
                          form))
                   (prelude:macro-function fn-symbol env))
                  form))
           (core:car form)
           (core:cdr form))
          form)))

;;;
;;; macroexpand: expand a form until it is no longer a macro call
;;;
(core:intern prelude "macroexpand"
   (:lambda (form env)
     ((:lambda (expanded)
          (:if (core:eq form expanded)
             expanded
             (prelude:macroexpand expanded env)))
       (prelude:macroexpand-1 form env))))

;;;
;;; macro-function: map a symbol to a macro function
;;;
;;; acts as a predicate
;;;
(core:intern prelude "macro-function"
   (:lambda (symbol env)
      (:if (core:eq :symbol (core:type-of symbol))
           ((:lambda (macro)
               (:if macro
                    (core:symbol-value macro)
                    ()))
            (prelude:%find-macro symbol env))
          ())))
