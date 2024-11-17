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
;;;  (:form    . :listp)      frame lambda
;;;  (:arity   . :fixnum)     frame lambda number of required args
;;;
(core:def-type "%closure"
    '((:require . :listp)
      (:rest    . :symbolp)
      (:arity   . :fixnum)
      (:form    . :listp)
      (:env     . :listp)))

(mu:intern core "%closure-prop"
   (:lambda (prop function)
     (mu:cdr (core:%type-ref prop function))))

(mu:intern core "%make-closure"
   (:lambda (requires rest-sym form env)
     (core:%make-core-type "%closure"
       `(,(mu:cons :require   requires)
          ,(mu:cons :rest     rest-sym)
          ,(mu:cons :form     form)
          ,(mu:cons :arity    (mu:sub (mu:length requires) (:if rest-sym 1 0)))
          ,(mu:cons :env      env)))))

;;;
;;; predicates
;;;
(mu:intern core "%closurep"
   (:lambda (function)
     (:if (core:%core-type-p function)
          (mu:eq '%closure (core:type-of function))
          ())))

(mu:intern core "functionp"
   (:lambda (function)
     (:if (mu:eq :func (mu:type-of function))
          :t
          (core:%closurep function))))

;;;
;;; functions 
;;;
(mu:intern core "%apply"
   (:lambda (closure frame arg-list-form)
     ((:lambda (env)
        (mu:apply frame arg-list-form))
     (core:%closure-prop :env closure))))

(mu:intern core "apply"
   (:lambda (function arg-list)
     (:if (core:functionp function)
          (:if (core:%closurep function)
               (core:%apply
                function
                (core:compile (core:%closure-prop :form function))
                (mu:eval (core:%compile-lambda-arg-list function arg-list ())))
               (mu:apply function arg-list))
          (core:raise function 'core:apply "not a function designator"))))

;;;
;;; compile closure
;;;
(mu:intern core "%compile-closure"
   (:lambda (lambda body env)
     (:if (core:%findl-if (:lambda (el) (core:null (mu:eq :symbol (mu:type-of el)))) lambda)
          (core:raise lambda 'core:%compile-closure "malformed lambda expression")
          ((:lambda (desc)
             ((:lambda (body)
                (:if (core:logor (core:fixnump desc) (core:null body))
                     (core:%make-closure lambda () `(:lambda ,lambda ,@body) ())
                     ((:lambda ()
                        ;;; (core:warn `(:lambda ,(mu:car desc) ,@(mu:cdr desc)) "compile-closure")
                        (core:%make-closure (mu:car desc) (mu:cdr desc) `(:lambda ,(mu:car desc) ,@body) ())))))
              (core:%mapcar (:lambda (form) (core:%compile form env)) body)))
           (core:%foldl
            (:lambda (el acc)
              (:if (core:%numberp acc)
                   (:if (mu:eq '&rest el)
                        (:if (mu:eq (mu:length lambda) (mu:add 2 acc))
                             (mu:cons
                              `(,@(core:%dropr lambda 2) ,@(core:%dropl lambda (core:1- (mu:length lambda))))
                              (mu:nth (core:1- (mu:length lambda)) lambda))
                             (core:raise lambda 'core:%compile-closure "rest botch"))
                        (core:1+ acc))
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
                                       (core:%findl-if
                                        (:lambda (lambda)
                                           (core:%findl-if
                                            (:lambda (symbol) (mu:eq symbol form))
                                            (core:%type-ref :require lambda)))
                                        env))))
                                 (core:%findl-if (:lambda (symbol) (mu:eq symbol form)) bound-symbols))
                             acc)
                        acc)))
           ()
           body))
       (core:%type-ref :require lambda))))
|#
