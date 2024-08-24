;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; closures
;;;

;;;
;;; implementation
;;;
(core:def-type "%closure"
    '((:lambda  . :struct)
      (:arity   . :fixnum)
      (:fn      . :func)
      (:env     . :listp)))

(mu:intern core "%closure-prop"
   (:lambda (prop closure)
      (:if (core:findl (:lambda (closure-prop) (mu:eq prop closure-prop)) '(:lambda :arity :fn :env))
           ((:lambda (ref)
               ;;; (core:raise-when core:null prop 'core:%function-prop "property not bound")
               (mu:cdr ref))
            (core:%type-ref prop closure))
           (core:%lambda-prop prop (core:%closure-prop :lambda closure)))))

;;;
;;; predicates
;;;
(mu:intern core "%closurep"
   (:lambda (fn)
     (core:%andf
      (core:%core-type-p fn)
      (mu:eq '%closure (core:type-of fn)))))

(mu:intern core "functionp"
   (:lambda (fn)
      (:if (mu:eq :func (mu:type-of fn))
           :t
           (core:%closurep fn))))

;;;
;;; [*closure descriptor*] #s(:<core> :%fn (lambda nargs fn env))
;;;
;;; lambda: lambda type
;;; nargs:  number of reqs plus rest
;;; fn:     mu function implementation
;;; env:    dynamic environment, list of frames
;;;
(mu:intern core "%make-closure"
   (:lambda (lambda fn env)
      ((:lambda (nargs)
          (core:%make-type "%closure"
             `(,(mu:cons :lambda lambda)
               ,(mu:cons :arity  nargs)
               ,(mu:cons :fn     fn)
               ,(mu:cons :env    env))))
       (mu:difference
        (mu:length (core:%lambda-prop :bound lambda))
        (:if (core:%lambda-prop :rest lambda) 1 0)))))

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
                        (:if (core:uninternedp elt)
                             ((:lambda (symbol)
                                 (:if symbol
                                      acc
                                      ((:lambda (binding-lambda)
                                          (:if binding-lambda
                                                `(,(mu:cons form binding-lambda) ,@acc)
                                                acc))
                                       (core:findl
                                        (:lambda (lambda)
                                           (core:findl
                                            (:lambda (symbol) (mu:eq symbol form))
                                            (core:%type-ref :bound lambda)))
                                        env))))
                                 (core:findl (:lambda (symbol) (mu:eq symbol form)) bound-symbols))
                             acc)
                        acc)))
           ()
           body))
       (core:%type-ref :bound lambda))))

;;;
;;; compile closed lambda to mu function
;;;
(mu:intern core "%compile-function"
   (:lambda (form env)
      ((:lambda (lambda)
          ((:lambda (closure)
             (core:%make-closure lambda closure ()))
           (mu:compile
            `(:lambda
              ,(core:%lambda-prop :bound lambda)
              ,(core:%compile-lambda-body lambda (mu:nthcdr 2 form) env)))))
       (core:%compile-lambda-desc (mu:nth 1 form) env))))
