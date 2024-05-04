;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lambda form
;;;
;;; [*lambda form*]
;;; (lambda ([symbol ...] [&rest rest-symbol]) . body)
;;;

;;;
;;; [*lambda descriptor*] #s(:<type> :lambda (bound rest lexenv))
;;;
;;; bound:  bound symbols
;;; rest:   rest symbol or nil
;;; lexenv: lexical environment, list of lambdas
;;;
(lib:intern prelude "%lambdap"
   (:lambda (fn)
      (lib:eq (prelude:type-of fn) 'lambda)))

(lib:intern prelude "%make-lambda"
   (:lambda (lambda-list rest-sym env)
      (prelude:%make-type "lambda"
         `(,(lib:cons :bound  lambda-list)
           ,(lib:cons :rest   rest-sym)
           ,(lib:cons :lexenv env)))))

(lib:intern prelude "%lambda-prop"
   (:lambda (prop lambda)
      (:if (prelude:findl (:lambda (lambda-prop) (lib:eq prop lambda-prop)) '(:bound :rest :lexenv))
           ((:lambda (ref)
               ;;; (prelude:raise-when prelude:null prop 'lambda-prop "property not bound")
               (lib:cdr ref))
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
(lib:intern prelude "%compile-lambda-list"
   (:lambda (lambda env)
      (:if (prelude:findl (:lambda (el) (prelude:null (lib:eq :symbol (lib:type-of el)))) lambda)
           (prelude:raise lambda 'prelude:%compile-lambda-list "list syntax")
           ((:lambda (desc)
               (:if (prelude:fixnump desc)
                    (prelude:%make-lambda lambda () env)
                    desc))
            (prelude:%foldl
             (:lambda (el acc)
                (:if (prelude:%lambdap acc)
                     acc
                     (:if (lib:eq '&rest el)
                          (:if (lib:eq (lib:length lambda) (lib:fx-add 2 acc))
                               (prelude:%make-lambda
                                `(,@(prelude:dropr lambda 2) ,@(prelude:dropl lambda (lib:fx-sub (lib:length lambda) 1)))
                                (lib:nth (lib:fx-sub (lib:length lambda) 1) lambda)
                                env)
                               (prelude:raise lambda 'prelude:%compile-lambda-list "rest botch"))
                          (lib:fx-add 1 acc))))
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
(lib:intern prelude "%compile-lambda-body"
   (:lambda (lambda-desc body env)
      (:if (prelude:null body)
           ()
           ((:lambda (env)
               (prelude:maplist
                (:lambda (body-cdr)
                   (:if (lib:cdr body-cdr)
                        ((:lambda (value)
                            (prelude:%compile value env))
                            (lib:car body-cdr))
                        ((:lambda (form)
                            (:if (prelude:consp form)
                                 (:if (lib:eq 'lambda (lib:car form))
                                      (prelude:%compile-function form env)
                                      (prelude:%compile form env))
                                 (prelude:%compile form env)))
                         (lib:car body-cdr))))
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
(lib:intern prelude "%frame-ref"
  (:lambda (symbol env)
    (prelude:%foldl
     (:lambda (frame acc)
       (:if acc
            acc
            ((:lambda (offset)
               (:if offset
                    (lib:compile
                     `(lib:fr-ref
                       ,(lib:cdr (prelude:%type-ref :func frame))
                       ,offset))
                    ()))
            (prelude:positionl
             (:lambda (el) (lib:eq el symbol))
             (lib:cdr (prelude:%type-ref :bound frame))))))
     ()
     env)))
|#
