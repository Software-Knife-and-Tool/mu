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
;;; env deftype
;;;

;;;
;;;  (:frame   . :fixnum)     frame function id
;;;  (:symbols . :listp)      lexical symbols
;;;  (:name    . :symbol)     closure symbol
;;;
(core:%deftype "%env"
    '((:frame   . :fixnum)
      (:symbols . :listp)
      (:name    . :symbol)))

(mu:intern core "%env-prop"
   (:lambda (prop env)
     (mu:cdr (core:%type-prop prop env))))

(mu:intern core "%make-env"
   (:lambda (frame symbols name)
     (core:%make-type "%closure"
       `(,(mu:cons :frame     frame)
          ,(mu:cons :symbols  symbols)
          ,(mu:cons :name     name)))))

(mu:intern core "%envp"
   (:lambda (function)
     (:if (core:%typep function)
          (mu:eq '%env (core:type-of function))
          ())))

;;;
;;; predicates
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
     (:if (core:%find-if (:lambda (el) (core:null (mu:eq :symbol (mu:type-of el)))) lambda)
          (core:%raise lambda 'core:%compile-closure "malformed lambda expression")
          ((:lambda (desc)
             ((:lambda (env)
                ((:lambda (body)
                   ((:lambda (closure)
                      (core:%add-frame (core:%env-prop :name (mu:car env)) (core:%closure-prop :mu closure))
                      closure)
                    (core:%make-closure (mu:car desc) (mu:cdr desc) `(:lambda ,(mu:car desc) ,@body) () ())))
                 (core:%mapcar
                  (:lambda (form)
                    (core:%compile form env))
                  body)))
              (mu:cons (core:%make-env (mu:length env) (mu:car desc) (core:gensym)) env)))
          ((:lambda (desc)
             (:if (core:%or (core:fixnump desc) (core:null body))
                  (core:%list lambda)
                  desc))
           (core:%foldl
            (:lambda (el acc)
              (:if (core:numberp acc)
                   (:if (mu:eq '&rest el)
                        (:if (mu:eq (mu:length lambda) (mu:add 2 acc))
                             (mu:cons
                              `(,@(core:%dropr lambda 2) ,@(core:%dropl lambda (mu:sub (mu:length lambda) 1)))
                              (mu:nth (mu:sub (mu:length lambda) 1) lambda))
                             (core:%raise lambda 'core:%compile-closure "rest botch"))
                        (mu:add 1 acc))
                   acc))
            0
            lambda))))))

;;;
;;; eval
;;;
(mu:intern core "eval"
   (:lambda (form)
     (mu:eval (core:compile form))))

;;;
;;; apply 
;;;
(mu:intern core "apply"
   (:lambda (function arg-list)
     (:if (core:%genericp function)
          (core:%apply-generic function arg-list)
          (:if (core:functionp function)
               (:if (core:%closurep function)
                    (core:%apply
                     function
                     (core:%closure-prop :mu function)
                     (mu:eval (core:%compile-lambda-arg-list function arg-list ())))
                    (mu:apply function arg-list))
               (core:%raise function 'core:apply "not a function designator")))))

(mu:intern core "%apply"
   (:lambda (closure frame arg-list-form)
     ((:lambda (env)
        (mu:apply frame arg-list-form))
      (core:%closure-prop :env closure))))

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
                                       (core:%find-if
                                        (:lambda (lambda)
                                           (core:%find-if
                                            (:lambda (symbol) (mu:eq symbol form))
                                            (core:%type-prop :require lambda)))
                                        env))))
                                 (core:%find-if (:lambda (symbol) (mu:eq symbol form)) bound-symbols))
                             acc)
                        acc)))
           ()
           body))
       (core:%type-prop :require lambda))))
|#
