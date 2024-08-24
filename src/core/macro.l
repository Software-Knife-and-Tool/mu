;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; macro definition, compilation, and expansion
;;;
(mu:intern core "%gen-macro-ns"
   (:lambda ()
      (mu:make-namespace (mu:symbol-name (core:genkeyword #\%)))))

(mu:intern core "%defmacro"
   (:lambda (symbol macro-function)
      ((:lambda (symbol-ns-name)
         ((:lambda (macro-ns)
            (:if macro-ns
                 (mu:intern (mu:find-namespace (mu:symbol-name macro-ns)) symbol-ns-name macro-function)
                 (mu:intern
                     ((:lambda (ns)
                        (mu:intern core:%macro-ns% symbol-ns-name ns)
                        ns)
                      (core:%gen-macro-ns))
                     (mu:symbol-name symbol)
                     macro-function))
            symbol)
          (mu:find core:%macro-ns% symbol-ns-name)))
       (mu:namespace-name (mu:symbol-namespace symbol)))))

(mu:intern core "%compile-macro-call"
   (:lambda (macro-symbol arg-list env)
       ((:lambda (macro-function)
          ((:lambda (expanded-form)
             (core:%compile expanded-form env))
           (:if (core:%closurep macro-function)
                (core:%fn-apply-quoted
                 macro-function
                 (mu:eval (core:%compile-lambda-arg-list macro-function arg-list env)))
                (mu:apply macro-function arg-list))))
      (core:macro-function macro-symbol env))))

;;;
;;; functions
;;;
(mu:intern core "macroexpand-1"
   (:lambda (form env)
     (:if (core:consp form)
          ((:lambda (fn-symbol arg-list)
             (:if (mu:eq :symbol (mu:type-of fn-symbol))
                  ((:lambda (macro-function)
                     (:if macro-function
                          (:if (core:%closurep macro-function)
                               (core:%fn-apply-quoted macro-function
                                                      (core:%quoted-lambda-arg-list macro-function arg-list))
                               (mu:apply macro-function arg-list))
                          form))
                   (core:macro-function fn-symbol env))
                  form))
           (mu:car form)
           (mu:cdr form))
          form)))

(mu:intern core "macroexpand"
   (:lambda (form env)
     ((:lambda (expanded)
          (:if (mu:eq form expanded)
             expanded
             (core:macroexpand expanded env)))
       (core:macroexpand-1 form env))))

(mu:intern core "macro-function"
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
             (mu:find core:%macro-ns% (mu:namespace-name (mu:symbol-namespace symbol)))
             (mu:symbol-name symbol)))
            ())))
