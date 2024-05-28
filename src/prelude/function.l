;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; closures
;;;
;;;
;;; predicates
;;;
(crux:intern prelude "%prelude-function-p"
   (:lambda (fn)
     (prelude:%andf
      (prelude:%prelude-type-p fn)
      (crux:eq '%fn (prelude:type-of fn)))))

(crux:intern prelude "functionp"
   (:lambda (fn)
      (:if (crux:eq :func (crux:type-of fn))
           :t
           (prelude:%prelude-function-p fn))))

;;;
;;; [*function descriptor*] #s(:<type> :%fn (lambda narg-list fn env))
;;;
;;; lambda: lambda type
;;; nargs:  number of reqs plus rest
;;; fn:     implementation
;;; env:    dynamic environment, list of frames
;;;
;;; function
;;;
(crux:intern prelude "%make-fn"
   (:lambda (lambda fn env)
      ((:lambda (nargs)
          (prelude:%make-type "%fn"
             `(,(crux:cons :lambda lambda)
               ,(crux:cons :arity  nargs)
               ,(crux:cons :fn     fn)
               ,(crux:cons :env    env))))
       (crux:difference
        (crux:length (prelude:%lambda-prop :bound lambda))
        (:if (prelude:%lambda-prop :rest lambda) 1 0)))))

(crux:intern prelude "%fn-prop"
   (:lambda (prop fn)
      (:if (prelude:findl (:lambda (fn-prop) (crux:eq prop fn-prop)) '(:lambda :arity :fn :env))
           ((:lambda (ref)
               ;;; (prelude:raise-when prelude:null prop 'prelude:%function-prop "property not bound")
               (crux:cdr ref))
            (prelude:%type-ref prop fn))
           (prelude:%lambda-prop prop (prelude:%fn-prop :lambda fn)))))

;;;
;;; create closed environment
;;;
;;; retruns env
;;;
(crux:intern prelude "%closure-env"
   (:lambda (lambda body env)
      ((:lambda (bound-symbols)
          (prelude:%foldl
           (:lambda (elt acc)
              (:if (prelude:consp elt)
                   ((:lambda (function-form)
                       (:if (prelude:consp function-form)
                            `(,@(prelude:%closure-env lambda function-form env) ,@acc)
                            `(,@(prelude:%closure-env lambda (crux:cdr elt) env) ,@acc)))
                    (crux:car elt))
                   (:if (crux:eq :symbol (crux:type-of elt))
                        (:if (prelude:uninternedp elt)
                             ((:lambda (symbol)
                                 (:if symbol
                                      acc
                                      ((:lambda (binding-lambda)
                                          (:if binding-lambda
                                                `(,(crux:cons form binding-lambda) ,@acc)
                                                acc))
                                       (prelude:findl
                                        (:lambda (lambda)
                                           (prelude:findl
                                            (:lambda (symbol) (crux:eq symbol form))
                                            (prelude:%type-ref :bound lambda)))
                                        env))))
                                 (prelude:findl (:lambda (symbol) (crux:eq symbol form)) bound-symbols))
                             acc)
                        acc)))
           ()
           body))
       (prelude:%type-ref :bound lambda))))

;;;
;;; %close-env = > closure
;;;
;;; crux:frames => list of [*frame descriptor*] (function . vector)
;;; env => list of (symbol . lambda)
;;;
(crux:intern prelude "%close-env"
  (:lambda (closure env)
     ((:lambda (lambda fn env)
         (prelude:%make-fn lambda fn env))
      (prelude:%make-lambda
       (prelude:%lambda-prop :bound fn)
       (prelude:%lambda-prop :rest fn)
       (crux:frames))
      (prelude:%fn-prop :fn fn))))

;;;
;;; compile closed lambda => function
;;;
(crux:intern prelude "%compile-function"
   (:lambda (form env)
      ((:lambda (lambda)
          ((:lambda (fn)
              (prelude:%make-fn lambda fn ()))
           (crux:compile
            `(:lambda
              ,(prelude:%lambda-prop :bound lambda)
              ,(prelude:%compile-lambda-body lambda (crux:nthcdr 2 form) env)))))
       (prelude:%compile-lambda-list (crux:nth 1 form) env))))
