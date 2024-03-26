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
(libcore:intern :prelude "%lambdap"
   (:lambda (fn)
      (libcore:eq (prelude:type-of fn) 'lambda)))

(libcore:intern :prelude "%make-lambda"
   (:lambda (lambda-list rest-sym env)
      (prelude:%make-type "lambda"
         `(,(libcore:cons :bound  lambda-list)
           ,(libcore:cons :rest   rest-sym)
           ,(libcore:cons :lexenv env)))))

(libcore:intern :prelude "%lambda-prop"
   (:lambda (prop lambda)
      (:if (prelude:findl (:lambda (lambda-prop) (libcore:eq prop lambda-prop)) '(:bound :rest :lexenv))
           ((:lambda (ref)
               ;;; (prelude:raise-when prelude:null prop 'lambda-prop "property not bound")
               (libcore:cdr ref))
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
(libcore:intern :prelude "%compile-lambda-list"
   (:lambda (lambda env)
      (:if (prelude:findl (:lambda (el) (prelude:null (libcore:eq :symbol (libcore:type-of el)))) lambda)
           (prelude:raise lambda 'prelude:%compile-lambda-list "list syntax")
           ((:lambda (desc)
               (:if (prelude:fixnump desc)
                    (prelude:%make-lambda lambda () env)
                    desc))
            (prelude:%foldl
             (:lambda (el acc)
                (:if (prelude:%lambdap acc)
                     acc
                     (:if (libcore:eq '&rest el)
                          (:if (libcore:eq (libcore:length lambda) (libcore:fx-add 2 acc))
                               (prelude:%make-lambda
                                `(,@(prelude:dropr lambda 2) ,@(prelude:dropl lambda (libcore:fx-sub (libcore:length lambda) 1)))
                                (libcore:nth (libcore:fx-sub (libcore:length lambda) 1) lambda)
                                env)
                               (prelude:raise lambda 'prelude:%compile-lambda-list "rest botch"))
                          (libcore:fx-add 1 acc))))
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
(libcore:intern :prelude "%compile-lambda-body"
   (:lambda (lambda-desc body env)
      (:if (prelude:null body)
           ()
           ((:lambda (env)
               (prelude:maplist
                (:lambda (body-cdr)
                   (:if (libcore:cdr body-cdr)
                        ((:lambda (value)
                            (prelude:%compile value env))
                            (libcore:car body-cdr))
                        ((:lambda (form)
                            (:if (prelude:consp form)
                                 (:if (libcore:eq 'lambda (libcore:car form))
                                      (prelude:%compile-function form env)
                                      (prelude:%compile form env))
                                 (prelude:%compile form env)))
                         (libcore:car body-cdr))))
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
(libcore:intern :prelude "%frame-ref"
  (:lambda (symbol env)
    (prelude:%foldl
     (:lambda (frame acc)
       (:if acc
            acc
            ((:lambda (offset)
               (:if offset
                    (libcore:compile
                     `(libcore:fr-ref
                       ,(libcore:cdr (prelude:%type-ref :func frame))
                       ,offset))
                    ()))
            (prelude:positionl
             (:lambda (el) (libcore:eq el symbol))
             (libcore:cdr (prelude:%type-ref :bound frame))))))
     ()
     env)))
|#
