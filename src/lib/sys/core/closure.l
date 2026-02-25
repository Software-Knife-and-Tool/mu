;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; closures
;;;

;;;
;;; closure deftype
;;;

;;;
;;;  (:require . :listp)      closure required arg symbols
;;;  (:rest    . :symbolp)    closure rest arg symbol
;;;  (:env     . :listp)      closure captured environment
;;;  (:mu      . :func)       closure implementation function
;;;  (:arity   . :fixnum)     frame lambda number of required args
;;;  (:lenv    . :listp)      compiled in lexical environment (symbol . (...))
;;;
(core:%deftype "%closure"
    '((:require . :listp)
      (:rest    . :symbolp)
      (:arity   . :fixnum)
      (:mu      . :func)
      (:env     . :listp)
      (:lenv    . :listp)))

(mu:intern core "%closure-prop"
   (:lambda (prop function)
     (mu:cdr (core:%type-prop prop function))))

(mu:intern core "%make-closure"
   (:lambda (requires rest-sym form env lenv)
     (core:%make-type "%closure"
       `(,(mu:cons :require   requires)
          ,(mu:cons :rest     rest-sym)
          ,(mu:cons :mu       (mu:compile form))
          ,(mu:cons :arity    (mu:sub (mu:length requires) (:if rest-sym 1 0)))
          ,(mu:cons :env      env)
          ,(mu:cons :lenv     lenv)))))

(mu:intern core "%closurep"
   (:lambda (function)
     (:if (core:%typep function)
          (mu:eq '%closure (core:type-of function))
          ())))

;;;
;;; predicate
;;;
(mu:intern core "functionp"
   (:lambda (function)
     (:if (mu:eq :func (mu:type-of function))
          :t
          (core:%closurep function))))

;;;
;;; compile closure
;;;
(mu:intern core "%compile-closure"
   (:lambda (lambda body env)
     (:if (core:find-if (:lambda (el) (core:null (mu:eq :symbol (mu:type-of el)))) lambda)
          (mu:raise lambda 'core:%compile-closure :syntax)
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
                              (mu:raise lambda 'core:%compile-closure :syntax))
                         (mu:add 1 acc))
                    acc))
             0
             lambda))))))

#|
;;;
;;; create closed environment
;;;
(mu:intern core "%closure-env"
   (:lambda (lambda body env)
      ((:lambda (bound-symbols)
          (core:foldl
           (:lambda (elt acc)
              (:if (core:consp elt)
                   ((:lambda (function-form)
                       (:if (core:consp function-form)
                            `(,@(core:%closure-env lambda function-form env) ,@acc)
                            `(,@(core:%closure-env lambda (mu:cdr elt) env) ,@acc)))
                    (mu:car elt))
                   (:if (mu:eq :symbol (mu:type-of elt))
                        (:if (core:%uninternedp elt)
                             ((:lambda (symbol)
                                 (:if symbol
                                      acc
                                      ((:lambda (binding-lambda)
                                          (:if binding-lambda
                                                (mu:cons (mu:cons form binding-lambda) acc)
                                                acc))
                                       (core:find-if
                                        (:lambda (lambda)
                                           (core:find-if
                                            (:lambda (symbol) (mu:eq symbol form))
                                            (core:%type-prop :require lambda)))
                                        env))))
                                 (core:find-if (:lambda (symbol) (mu:eq symbol form)) bound-symbols))
                             acc)
                        acc)))
           ()
           body))
       (core:%type-prop :require lambda))))
|#
