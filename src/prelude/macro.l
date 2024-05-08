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
(lib:intern prelude "%map-macro-symbol-ns"
   (:lambda (symbol)
      ((:lambda (ns)
          (lib:find prelude:%macro-ns% (lib:ns-name ns)))
       (lib:symbol-ns symbol))))

(lib:intern prelude "%make-macro-ns"
   (:lambda (ns)
       ((:lambda (macro-ns)
          (lib:intern prelude:%macro-ns% (lib:ns-name ns) macro-ns)
          macro-ns)
       (prelude:%gen-anon-ns))))

(lib:intern prelude "%defmacro"
   (:lambda (symbol macro-function)
      ((:lambda (macro-ns symbol-ns)
         (:if macro-ns
              (lib:intern macro-ns symbol-ns macro-function)
              (lib:intern (prelude:%make-macro-ns symbol-ns) (lib:symbol-name symbol) macro-function))
         symbol)
       (prelude:%map-macro-symbol-ns symbol)
       (lib:symbol-ns symbol))))

(lib:intern prelude "%find-macro"
    (:lambda (symbol env)
      ((:lambda (macro-ns symbol-name)
          (:if macro-ns
               (lib:find (lib:symbol-value macro-ns) symbol-name)
               ()))
       (lib:find prelude:%macro-ns% (lib:ns-name (lib:symbol-ns symbol)))
       (lib:symbol-name symbol))))

(lib:intern prelude "%compile-macro-call"
   (:lambda (macro-symbol arg-list env)
       ((:lambda (macro-function)
          ((:lambda (expanded-form)
             (prelude:%compile expanded-form env))
           (:if (prelude:%prelude-function-p macro-function)
                (prelude:%fn-apply-quoted
                 macro-function
                 (lib:eval (prelude:%compile-lambda-arg-list macro-function arg-list env)))
                (lib:apply macro-function arg-list))))
      (prelude:macro-function macro-symbol env))))

;;;
;;; macroexpand-1 expands a macro call once.
;;;
(lib:intern prelude "macroexpand-1"
   (:lambda (form env)
     (:if (prelude:consp form)
          ((:lambda (fn-symbol arg-list)
             (:if (lib:eq :symbol (lib:type-of fn-symbol))
                  ((:lambda (macro-function)
                     (:if macro-function
                          (:if (prelude:%prelude-function-p macro-function)
                               (prelude:%fn-apply-quoted macro-function (prelude:%quoted-lambda-arg-list macro-function arg-list))
                               (lib:apply macro-function arg-list))
                          form))
                   (prelude:macro-function fn-symbol env))
                  form))
           (lib:car form)
           (lib:cdr form))
          form)))

;;;
;;; macroexpand: expand a form until it is no longer a macro call
;;;
(lib:intern prelude "macroexpand"
   (:lambda (form env)
     ((:lambda (expanded)
          (:if (lib:eq form expanded)
             expanded
             (prelude:macroexpand expanded env)))
       (prelude:macroexpand-1 form env))))

;;;
;;; macro-function: map a symbol to a macro function
;;;
;;; acts as a predicate
;;;
(lib:intern prelude "macro-function"
   (:lambda (symbol env)
      (:if (lib:eq :symbol (lib:type-of symbol))
           ((:lambda (macro)
               (:if macro
                    (lib:symbol-value macro)
                    ()))
            (prelude:%find-macro symbol env))
          ())))
