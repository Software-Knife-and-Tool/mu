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
(mu:intern prelude "%defmacro"
   (:lambda (symbol macro-function)
      ((:lambda (symbol-ns-name)
         ((:lambda (macro-ns)
            (:if macro-ns
                 (mu:intern (mu:find-namespace (mu:symbol-name macro-ns)) symbol-ns-name macro-function)
                 (mu:intern
                     ((:lambda (ns)
                        (mu:intern prelude:%macro-ns% symbol-ns-name ns)
                        ns)
                      (prelude:%gen-anon-ns))
                     (mu:symbol-name symbol)
                     macro-function))
            symbol)
          (mu:find prelude:%macro-ns% symbol-ns-name)))
       (mu:namespace-name (mu:symbol-namespace symbol)))))

(mu:intern prelude "%compile-macro-call"
   (:lambda (macro-symbol arg-list env)
       ((:lambda (macro-function)
          ((:lambda (expanded-form)
             (prelude:%compile expanded-form env))
           (:if (prelude:%prelude-function-p macro-function)
                (prelude:%fn-apply-quoted
                 macro-function
                 (mu:eval (prelude:%compile-lambda-arg-list macro-function arg-list env)))
                (mu:apply macro-function arg-list))))
      (prelude:macro-function macro-symbol env))))

;;;
;;; macroexpand-1 expands a macro call once.
;;;
(mu:intern prelude "macroexpand-1"
   (:lambda (form env)
     (:if (prelude:consp form)
          ((:lambda (fn-symbol arg-list)
             (:if (mu:eq :symbol (mu:type-of fn-symbol))
                  ((:lambda (macro-function)
                     (:if macro-function
                          (:if (prelude:%prelude-function-p macro-function)
                               (prelude:%fn-apply-quoted macro-function (prelude:%quoted-lambda-arg-list macro-function arg-list))
                               (mu:apply macro-function arg-list))
                          form))
                   (prelude:macro-function fn-symbol env))
                  form))
           (mu:car form)
           (mu:cdr form))
          form)))

;;;
;;; macroexpand: expand a form until it is no longer a macro call
;;;
(mu:intern prelude "macroexpand"
   (:lambda (form env)
     ((:lambda (expanded)
          (:if (mu:eq form expanded)
             expanded
             (prelude:macroexpand expanded env)))
       (prelude:macroexpand-1 form env))))

;;;
;;; macro-function: map a symbol to a macro function
;;;
;;; acts as a predicate
;;;
(mu:intern prelude "macro-function"
   (:lambda (symbol env)
      (:if (mu:eq :symbol (mu:type-of symbol))
           ((:lambda (macro)
               (:if macro
                    (mu:symbol-value macro)
                    ()))
            ((:lambda (macro-ns symbol-name)
               (:if macro-ns
                    (mu:find (mu:symbol-value macro-ns) symbol-name)
                    ()))
             (mu:find prelude:%macro-ns% (mu:namespace-name (mu:symbol-namespace symbol)))
             (mu:symbol-name symbol)))
            ())))
