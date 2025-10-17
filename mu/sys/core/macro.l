;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core macros
;;;
(mu:intern core "%defmacro"
   (:lambda (symbol macro-function)
     ((:lambda (symbol-ns symbol-name)
        (:if (mu:eq :unqual symbol-ns)
             (mu:intern core:%macros/ symbol-name macro-function)
             (mu:raise-from symbol-ns 'core:%defmacro :range))
        symbol)
      (mu:symbol-namespace symbol)
      (mu:symbol-name symbol))))

(mu:intern core "%compile-macro-call"
   (:lambda (macro-symbol arg-list env)
     (core:%compile (core:macroexpand (mu:cons macro-symbol arg-list) env) env)))

;;;
;;; compile macro closure
;;;
(mu:intern core "%compile-macro-closure"
   (:lambda (lambda body env)
     (:if (core:find-if (:lambda (el) (core:null (mu:eq :symbol (mu:type-of el)))) lambda)
          (mu:raise-from lambda 'core:%compile-macro-closure :syntax)
          ((:lambda (desc)
             ((:lambda (env)
                ((:lambda (body)
                   ((:lambda (closure)
                      (core:%add-frame (core:%env-prop :name (mu:car env)) (core:%closure-prop :mu closure))
                      closure)
                    (core:%make-closure (mu:car desc) (mu:cdr desc) `(:lambda ,(mu:car desc) ,@body) () ())))
                 (core:mapcar
                  (:lambda (form)
                    (core:%compile form env))
                  body)))
              (mu:cons (core:%make-env (mu:length env) (mu:car desc) (core:gensym)) env)))
          ((:lambda (desc)
             (:if (core:%or (core:fixnump desc) (core:null body))
                  (core:%list lambda)
                  desc))
           (core:foldl
            (:lambda (el acc)
              (:if (core:numberp acc)
                   (:if (mu:eq '&rest el)
                        (:if (mu:eq (mu:length lambda) (mu:add 2 acc))
                             (mu:cons
                              `(,@(core:dropr lambda 2) ,@(core:dropl lambda (mu:sub (mu:length lambda) 1)))
                              (mu:nth (mu:sub (mu:length lambda) 1) lambda))
                             (mu:raise-from lambda 'core:%compile-macro-closure :syntax))
                        (mu:add 1 acc))
                   acc))
            0
            lambda))))))

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
           (mu:find core:%macros/ (mu:symbol-name symbol)))
          (mu:raise-from symbol 'core:macro-function :type))))
