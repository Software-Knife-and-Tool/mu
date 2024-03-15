;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; closures
;;;
;;;
;;; predicates
;;;
(mu:intern :prelude "%prelude-function-p"
   (:lambda (fn)
      (mu:eq 'funct (prelude:type-of fn))))

(mu:intern :prelude "functionp"
   (:lambda (function)
      (:if (mu:eq :func (mu:type-of function))
           :t
           (mu:eq 'funct (prelude:type-of function)))))

;;;
;;; [*function descriptor*] #s(:<type> :funct (lambda narg-list fn env))
;;;
;;; lambda: lambda type
;;; nargs:  number of reqs plus rest
;;; fn:     implementation
;;; env:    dynamic environment, list of frames
;;;
;;; function
;;;
(mu:intern :prelude "%make-funct"
   (:lambda (lambda fn env)
      ((:lambda (nargs)
          (prelude:%make-type "funct"
             `(,(mu:cons :lambda lambda)
               ,(mu:cons :arity  nargs)
               ,(mu:cons :fn     fn)
               ,(mu:cons :env    env))))
       (mu:fx-sub
        (mu:length (prelude:%lambda-prop :bound lambda))
        (:if (prelude:%lambda-prop :rest lambda) 1 0)))))

(mu:intern :prelude "%funct-prop"
   (:lambda (prop funct)
      (:if (prelude:findl (:lambda (funct-prop) (mu:eq prop funct-prop)) '(:lambda :arity :fn :env))
           ((:lambda (ref)
               ;;; (prelude:raise-when prelude:null prop 'prelude:%function-prop "property not bound")
               (mu:cdr ref))
            (prelude:%type-ref prop funct))
           (prelude:%lambda-prop prop (prelude:%funct-prop :lambda funct)))))

;;;
;;; create closed environment
;;;
;;; retruns env
;;;
(mu:intern :prelude "%closure-env"
   (:lambda (lambda body env)
      ((:lambda (bound-symbols)
          (prelude:%foldl
           (:lambda (elt acc)
              (:if (prelude:consp elt)
                   ((:lambda (function-form)
                       (:if (prelude:consp function-form)
                            `(,@(prelude:%closure-env lambda function-form env) ,@acc)
                            `(,@(prelude:%closure-env lambda (mu:cdr elt) env) ,@acc)))
                    (mu:car elt))
                   (:if (mu:eq :symbol (mu:type-of elt))
                        (:if (prelude:uninternedp elt)
                             ((:lambda (symbol)
                                 (:if symbol
                                      acc
                                      ((:lambda (binding-lambda)
                                          (:if binding-lambda
                                                `(,(mu:cons form binding-lambda) ,@acc)
                                                acc))
                                       (prelude:findl
                                        (:lambda (lambda)
                                           (prelude:findl
                                            (:lambda (symbol) (mu:eq symbol form))
                                            (prelude:%type-ref :bound lambda)))
                                        env))))
                                 (prelude:findl (:lambda (symbol) (mu:eq symbol form)) bound-symbols))
                             acc)
                        acc)))
           ()
           body))
       (prelude:%type-ref :bound lambda))))

;;;
;;; %close-env = > closure
;;;
;;; mu:frames => list of [*frame descriptor*] (function . vector)
;;; env => list of (symbol . lambda)
;;;
(mu:intern :prelude "%close-env"
  (:lambda (closure env)
     ((:lambda (lambda fn env)
         (prelude:%make-funct lambda fn env))
      (prelude:%make-lambda
       (prelude:%lambda-prop :bound fn)
       (prelude:%lambda-prop :rest fn)
       (mu:frames))
      (prelude:%funct-prop :fn fn))))

;;;
;;; compile closed lambda => function
;;;
(mu:intern :prelude "%compile-function"
   (:lambda (form env)
      ((:lambda (lambda)
          ((:lambda (fn)
              (prelude:%make-funct lambda fn ()))
           (mu:compile
            `(:lambda
              ,(prelude:%lambda-prop :bound lambda)
              ,(prelude:%compile-lambda-body lambda (mu:nthcdr 2 form) env)))))
       (prelude:%compile-lambda-list (mu:nth 1 form) env))))
