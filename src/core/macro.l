;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; macro definition, compilation, and expansion
;;;
(mu:intern core "%defmacro"
   (:lambda (symbol macro-function)
      ((:lambda (symbol-ns symbol-name)
         (:if (mu:eq symbol-ns core:%null-ns%)
              (mu:intern core:%macro-ns% symbol-name macro-function)
              (core:raise symbol-ns 'core:%defmacro "illegal symbol"))
         symbol)
       (mu:symbol-namespace symbol)
       (mu:symbol-name symbol))))

(mu:intern core "%describe-fn"
   (:lambda (fn)
     (:if (core:%closurep fn)
          ((:lambda (lambda arity fn env)
             (core:eprint "" "%describe-fn:closure")
             (core:eprint lambda "    lambda")
             (core:eprint arity "    arity")
             (core:eprint fn "    fn")
             (core:eprint env "    env"))
           (core:%closure-prop :lambda fn)
           (core:%closure-prop :arity fn)
           (core:%closure-prop :fn fn)
           (core:%closure-prop :env fn))
          (core:print (mu:view fn) "%describe-fn:mu-fn"))))

(mu:intern core "%compile-macro-call"
   (:lambda (macro-symbol arg-list env)
     ((:lambda (macro-function)
        ;;; (core:%describe-fn macro-function)
        ;;; (core:warn arg-list "%compile-macro-call on")
        ((:lambda (expanded-form)
           (core:%compile expanded-form env))
         (:if (core:%closurep macro-function)
              (core:%fn-apply-quoted
               macro-function
               (mu:eval (core:%compile-lambda-arg-list macro-function arg-list env)))
              (mu:apply macro-function arg-list))))
      (core:%macro-function macro-symbol env))))

;;;
;;; functions
;;;
(mu:intern core "%macroexpand-1"
   (:lambda (form env)
     (:if (core:%consp form)
          ((:lambda (fn-symbol arg-list)
             (:if (mu:eq :symbol (mu:type-of fn-symbol))
                  ((:lambda (macro-function)
                     (:if macro-function
                          (:if (core:%closurep macro-function)
                               (core:%fn-apply-quoted macro-function
                                                      (core:%quoted-lambda-arg-list macro-function arg-list))
                               (mu:apply macro-function arg-list))
                          form))
                   (core:%macro-function fn-symbol env))
                  form))
           (mu:car form)
           (mu:cdr form))
          form)))

#|
(mu:intern core "%macroexpand"
   (:lambda (form env)
     ((:lambda (expanded)
          (:if (mu:eq form expanded)
             expanded
             (core:%macroexpand expanded env)))
       (core:%macroexpand-1 form env))))
|#

(mu:intern core "%macroexpand"
   (:lambda (form env)
     (mu:fix
      (:lambda (expanded)
        (core:%macroexpand-1 expanded env))
      form)))

(mu:intern core "%macro-function"
   (:lambda (symbol env)
     (:if (mu:eq :symbol (mu:type-of symbol))
          ((:lambda (macro)
             (:if macro
                  (mu:symbol-value macro)
                  ()))
           (mu:find core:%macro-ns% (mu:symbol-name symbol)))
          ())))
