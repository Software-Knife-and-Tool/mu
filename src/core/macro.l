;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core macros
;;;
(mu:intern core "%defmacro"
   (:lambda (symbol macro-function)
      ((:lambda (symbol-ns symbol-name)
         (:if (mu:eq symbol-ns mu:*null/*)
              (mu:intern core:*macros/* symbol-name macro-function)
              (core:%raise symbol-ns 'core:%defmacro "illegal symbol"))
         symbol)
       (mu:symbol-namespace symbol)
       (mu:symbol-name symbol))))

(mu:intern core "%compile-macro-call"
   (:lambda (macro-symbol arg-list env)
     (core:%compile (core:macroexpand (mu:cons macro-symbol arg-list) ()) env)))

;;;
;;; functions
;;;
(mu:intern core "macroexpand-1"
   (:lambda (form env)
     (:if (core:consp form)
          ((:lambda (macro-symbol arg-list)
             (:if (mu:eq :symbol (mu:type-of macro-symbol))
                  ((:lambda (macro-function)
                     (:if macro-function
                          (:if (core:%closurep macro-function)
                               (mu:apply
                                (core:%closure-prop :mu macro-function)
                                (mu:eval (core:%compile-quoted-lambda-arg-list macro-function arg-list env)))
                               (mu:apply macro-function arg-list))
                          form))
                   (core:macro-function macro-symbol env))
                  form))
           (mu:car form)
           (mu:cdr form))
          form)))

(mu:intern core "macroexpand"
   (:lambda (form env)
     (mu:fix
      (:lambda (expanded)
        (core:macroexpand-1 expanded env))
      form)))

(mu:intern core "macro-function"
   (:lambda (symbol env)
     (:if (core:symbolp symbol)
          ((:lambda (macro)
             (:if macro
                  (mu:symbol-value macro)
                  ()))
           (mu:find core:*macros/* (mu:symbol-name symbol)))
          (core:%raise symbol 'core:macro-function "not a symbol"))))
