;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; closures
;;;
;;;
;;; predicates
;;;
(lib:intern :prelude "%prelude-function-p"
   (:lambda (fn)
      (lib:eq 'funct (prelude:type-of fn))))

(lib:intern :prelude "functionp"
   (:lambda (function)
      (:if (lib:eq :func (lib:type-of function))
           :t
           (lib:eq 'funct (prelude:type-of function)))))

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
(lib:intern :prelude "%make-funct"
   (:lambda (lambda fn env)
      ((:lambda (nargs)
          (prelude:%make-type "funct"
             `(,(lib:cons :lambda lambda)
               ,(lib:cons :arity  nargs)
               ,(lib:cons :fn     fn)
               ,(lib:cons :env    env))))
       (lib:fx-sub
        (lib:length (prelude:%lambda-prop :bound lambda))
        (:if (prelude:%lambda-prop :rest lambda) 1 0)))))

(lib:intern :prelude "%funct-prop"
   (:lambda (prop funct)
      (:if (prelude:findl (:lambda (funct-prop) (lib:eq prop funct-prop)) '(:lambda :arity :fn :env))
           ((:lambda (ref)
               ;;; (prelude:raise-when prelude:null prop 'prelude:%function-prop "property not bound")
               (lib:cdr ref))
            (prelude:%type-ref prop funct))
           (prelude:%lambda-prop prop (prelude:%funct-prop :lambda funct)))))

;;;
;;; create closed environment
;;;
;;; retruns env
;;;
(lib:intern :prelude "%closure-env"
   (:lambda (lambda body env)
      ((:lambda (bound-symbols)
          (prelude:%foldl
           (:lambda (elt acc)
              (:if (prelude:consp elt)
                   ((:lambda (function-form)
                       (:if (prelude:consp function-form)
                            `(,@(prelude:%closure-env lambda function-form env) ,@acc)
                            `(,@(prelude:%closure-env lambda (lib:cdr elt) env) ,@acc)))
                    (lib:car elt))
                   (:if (lib:eq :symbol (lib:type-of elt))
                        (:if (prelude:uninternedp elt)
                             ((:lambda (symbol)
                                 (:if symbol
                                      acc
                                      ((:lambda (binding-lambda)
                                          (:if binding-lambda
                                                `(,(lib:cons form binding-lambda) ,@acc)
                                                acc))
                                       (prelude:findl
                                        (:lambda (lambda)
                                           (prelude:findl
                                            (:lambda (symbol) (lib:eq symbol form))
                                            (prelude:%type-ref :bound lambda)))
                                        env))))
                                 (prelude:findl (:lambda (symbol) (lib:eq symbol form)) bound-symbols))
                             acc)
                        acc)))
           ()
           body))
       (prelude:%type-ref :bound lambda))))

;;;
;;; %close-env = > closure
;;;
;;; lib:frames => list of [*frame descriptor*] (function . vector)
;;; env => list of (symbol . lambda)
;;;
(lib:intern :prelude "%close-env"
  (:lambda (closure env)
     ((:lambda (lambda fn env)
         (prelude:%make-funct lambda fn env))
      (prelude:%make-lambda
       (prelude:%lambda-prop :bound fn)
       (prelude:%lambda-prop :rest fn)
       (lib:frames))
      (prelude:%funct-prop :fn fn))))

;;;
;;; compile closed lambda => function
;;;
(lib:intern :prelude "%compile-function"
   (:lambda (form env)
      ((:lambda (lambda)
          ((:lambda (fn)
              (prelude:%make-funct lambda fn ()))
           (lib:compile
            `(:lambda
              ,(prelude:%lambda-prop :bound lambda)
              ,(prelude:%compile-lambda-body lambda (lib:nthcdr 2 form) env)))))
       (prelude:%compile-lambda-list (lib:nth 1 form) env))))
