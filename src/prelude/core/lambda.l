;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; (lambda ([symbol ...] [&rest rest-symbol]) . body)
;;;

;;;
;;; lambda descriptor: #s(:<type> :lambda (bound rest lexenv))
;;;
;;; bound:  bound symbols
;;; rest:   rest symbol or nil
;;; lexenv: lexical environment, list of lambdas
;;;
(mu:intern prelude "%lambdap"
    (:lambda (fn)
      (prelude:%andf
       (prelude:%prelude-type-p fn)
       (mu:eq '%lambda (prelude:type-of fn)))))

(mu:intern prelude "%make-lambda"
   (:lambda (lambda-list rest-sym env)
      (prelude:%make-type "%lambda"
         `(,(mu:cons :bound  lambda-list)
           ,(mu:cons :rest   rest-sym)
           ,(mu:cons :lexenv env)))))

(mu:intern prelude "%lambda-prop"
   (:lambda (prop lambda)
      (:if (prelude:findl (:lambda (lambda-prop) (mu:eq prop lambda-prop)) '(:bound :rest :lexenv))
           ((:lambda (ref)
               ;;; (prelude:raise-when prelude:null prop 'lambda-prop "property not bound")
               (mu:cdr ref))
            (prelude:%type-ref prop lambda))
           (prelude:raise prop 'prelude:%lambda-prop "not a lambda property"))))

;;;
;;; `%compile-lambda-list` compiles a lambda form to a lambda descriptor
;;;
;;; `lambda:` whole lambda form, with leading symbol
;;; `env:`    lexical environment
;;;
;;; `returns:` lambda descriptor
;;;
(mu:intern prelude "%compile-lambda-list"
   (:lambda (lambda env)
      (:if (prelude:findl (:lambda (el) (prelude:null (mu:eq :symbol (mu:type-of el)))) lambda)
           (prelude:raise lambda 'prelude:%compile-lambda-list "list syntax")
           ((:lambda (desc)
               (:if (prelude:fixnump desc)
                    (prelude:%make-lambda lambda () env)
                    desc))
            (prelude:%foldl
             (:lambda (el acc)
                (:if (prelude:%lambdap acc)
                     acc
                     (:if (mu:eq '&rest el)
                          (:if (mu:eq (mu:length lambda) (mu:sum 2 acc))
                               (prelude:%make-lambda
                                `(,@(prelude:dropr lambda 2) ,@(prelude:dropl lambda (mu:difference (mu:length lambda) 1)))
                                (mu:nth (mu:difference (mu:length lambda) 1) lambda)
                                env)
                               (prelude:raise lambda 'prelude:%compile-lambda-list "rest botch"))
                          (mu:sum 1 acc))))
             0
             lambda)))))

;;;
;;; `%compile-lambda-body` resolves body symbols with `prelude::compile-symbol`,
;;; uses the prelude compiler for everything else, and returns a compiled body.
;;;
;;; if a body returns a lambda expression, generate a closure body.
;;;
;;; `lambda-desc:` lambda descriptor
;;; `body:` list of body forms
;;; `env:` runtime lexical env
;;;
;;; `returns:` compiled body/closure
;;;
(mu:intern prelude "%compile-lambda-body"
   (:lambda (lambda-desc body env)
      (:if (prelude:null body)
           ()
           ((:lambda (env)
               (prelude:maplist
                (:lambda (body-cdr)
                   (:if (mu:cdr body-cdr)
                        ((:lambda (value)
                            (prelude:%compile value env))
                            (mu:car body-cdr))
                        ((:lambda (form)
                            (:if (prelude:consp form)
                                 (:if (mu:eq 'lambda (mu:car form))
                                      (prelude:%compile-function form env)
                                      (prelude:%compile form env))
                                 (prelude:%compile form env)))
                         (mu:car body-cdr))))
                body))
          `(,lambda-desc ,@env)))))

#|
;;;
;;; resolving body symbols
;;;
;;; *%frame-ref* looks up *symbol* in the lexical environment.
;;;
;;; if found return a compiled frame reference, otherwise *nil*.
;;;
;;; `symbol:` symbol to be compiled
;;; `env:` lexical compiled environment
;;;
;;; `returns:` *frame-ref* or *nil*
;;;
(mu:intern prelude "%frame-ref"
  (:lambda (symbol env)
    (prelude:%foldl
     (:lambda (frame acc)
       (:if acc
            acc
            ((:lambda (offset)
               (:if offset
                    (mu:compile
                     `(mu:frame-ref
                       ,(mu:cdr (prelude:%type-ref :func frame))
                       ,offset))
                    ()))
            (prelude:positionl
             (:lambda (el) (mu:eq el symbol))
             (mu:cdr (prelude:%type-ref :bound frame))))))
     ()
     env)))
|#
