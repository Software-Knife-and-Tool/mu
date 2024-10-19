;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core functions
;;;
(core:def-type "%core-function"
    '((:require . :listp)
      (:rest    . :symbolp)
      (:form    . :listp)
      (:arity   . :fixnum)
      (:mu      . :listp)
      (:env     . :listp)
      (:lexenv  . :listp)))

(mu:intern core "%core-function-prop"
   (:lambda (prop function)
     (mu:cdr (core:%type-ref prop function))))

(mu:intern core "core-function-prop"
   (:lambda (prop function)
     (:if (core:%findl
           (:lambda (fprop)
             (mu:eq prop fprop))
           '(:require :rest :form :arity :mu :env :lexenv))
          (core:%core-function-prop prop function)
          (core:raise prop 'core:%core-function-prop "not a function property"))))

(mu:intern core "%make-core-function"
   (:lambda (requires rest-sym form mu lexenv env)
     (core:%make-core-type "%core-function"
       `(,(mu:cons :require   requires)
          ,(mu:cons :rest     rest-sym)
          ,(mu:cons :form     form)
          ,(mu:cons :mu       mu)
          ,(mu:cons :arity    (mu:sub (mu:length requires) (:if rest-sym 1 0)))
          ,(mu:cons :env      env)
          ,(mu:cons :lexenv   lexenv)))))

;;;
;;; predicates
;;;
(mu:intern core "%core-function-p"
   (:lambda (function)
     (:if (core:%core-type-p function)
          (mu:eq '%core-function (core:type-of function))
          ())))

(mu:intern core "%closurep"
   (:lambda (function)
     (:if (core:%core-function-p function)
          (core:null (core:null (core:%core-function-prop :env function)))
          ())))

(mu:intern core "functionp"
   (:lambda (function)
     (core:logor
      (mu:eq :func (mu:type-of function))
      (core:%core-function-p function))))

;;;
;;; debugging
;;;
#|
(mu:intern core "%describe-core-function"
   (:lambda (fn)
     (:if (core:%closurep fn)
          ((:lambda (lambda arity fn env)
             (core:eprint "" "%describe-fn:closure")
             (core:eprint lambda "    lambda")
             (core:eprint arity "    arity")
             (core:eprint env "    env"))
           (core:%core-function-prop :lambda fn)
           (core:%core-function-prop :arity fn)
           (core:%core-function-prop :env fn))
          (core:print (mu:view fn) "%describe-fn:mu-fn"))))
|#

;;;
;;; constructor
;;;
(mu:intern core "compile-core-function"
   (:lambda (lambda body env)
     (:if (core:%findl (:lambda (el) (core:null (mu:eq :symbol (mu:type-of el)))) lambda)
          (core:raise lambda 'core:compile-core-function "lambda list syntax")
          ((:lambda (desc)
             ((:lambda (body)
                (:if (core:logor (core:%fixnump desc) (core:null body))
                     (core:%make-core-function lambda () body `(:lambda ,lambda ,@body) env ())
                     ((:lambda (lambda rest)
                        ((:lambda (mu-form)
                           (core:%make-core-function lambda rest body mu-form env ()))
                         `(:lambda ,lambda ,@body)))
                      (mu:car desc)
                      (mu:cdr desc))))
              (core:%mapcar (:lambda (form) (core:%compile form env)) body)))

           (core:%foldl
            (:lambda (el acc)
              (:if (core:%numberp acc)
                   (:if (mu:eq '&rest el)
                        (:if (mu:eq (mu:length lambda) (mu:add 2 acc))
                             (mu:cons
                              `(,@(core:%dropr lambda 2) ,@(core:%dropl lambda (mu:sub (mu:length lambda) 1)))
                              (mu:nth (mu:sub (mu:length lambda) 1) lambda))
                             (core:raise lambda 'core:compile-core-function "rest botch"))
                        (mu:add 1 acc))
                   acc))
            0
            lambda)))))

#|
;;;
;;; create closed environment
;;;
(mu:intern core "%closure-env"
   (:lambda (lambda body env)
      ((:lambda (bound-symbols)
          (core:%foldl
           (:lambda (elt acc)
              (:if (core:%consp elt)
                   ((:lambda (function-form)
                       (:if (core:%consp function-form)
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
                                       (core:%findl
                                        (:lambda (lambda)
                                           (core:%findl
                                            (:lambda (symbol) (mu:eq symbol form))
                                            (core:%type-ref :require lambda)))
                                        env))))
                                 (core:%findl (:lambda (symbol) (mu:eq symbol form)) bound-symbols))
                             acc)
                        acc)))
           ()
           body))
       (core:%type-ref :require lambda))))
|#
