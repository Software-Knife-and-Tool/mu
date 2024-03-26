;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; closures
;;;
;;;
;;; predicates
;;;
(libcore:intern :prelude "%prelude-function-p"
   (:lambda (fn)
      (libcore:eq 'funct (prelude:type-of fn))))

(libcore:intern :prelude "functionp"
   (:lambda (function)
      (:if (libcore:eq :func (libcore:type-of function))
           :t
           (libcore:eq 'funct (prelude:type-of function)))))

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
(libcore:intern :prelude "%make-funct"
   (:lambda (lambda fn env)
      ((:lambda (nargs)
          (prelude:%make-type "funct"
             `(,(libcore:cons :lambda lambda)
               ,(libcore:cons :arity  nargs)
               ,(libcore:cons :fn     fn)
               ,(libcore:cons :env    env))))
       (libcore:fx-sub
        (libcore:length (prelude:%lambda-prop :bound lambda))
        (:if (prelude:%lambda-prop :rest lambda) 1 0)))))

(libcore:intern :prelude "%funct-prop"
   (:lambda (prop funct)
      (:if (prelude:findl (:lambda (funct-prop) (libcore:eq prop funct-prop)) '(:lambda :arity :fn :env))
           ((:lambda (ref)
               ;;; (prelude:raise-when prelude:null prop 'prelude:%function-prop "property not bound")
               (libcore:cdr ref))
            (prelude:%type-ref prop funct))
           (prelude:%lambda-prop prop (prelude:%funct-prop :lambda funct)))))

;;;
;;; create closed environment
;;;
;;; retruns env
;;;
(libcore:intern :prelude "%closure-env"
   (:lambda (lambda body env)
      ((:lambda (bound-symbols)
          (prelude:%foldl
           (:lambda (elt acc)
              (:if (prelude:consp elt)
                   ((:lambda (function-form)
                       (:if (prelude:consp function-form)
                            `(,@(prelude:%closure-env lambda function-form env) ,@acc)
                            `(,@(prelude:%closure-env lambda (libcore:cdr elt) env) ,@acc)))
                    (libcore:car elt))
                   (:if (libcore:eq :symbol (libcore:type-of elt))
                        (:if (prelude:uninternedp elt)
                             ((:lambda (symbol)
                                 (:if symbol
                                      acc
                                      ((:lambda (binding-lambda)
                                          (:if binding-lambda
                                                `(,(libcore:cons form binding-lambda) ,@acc)
                                                acc))
                                       (prelude:findl
                                        (:lambda (lambda)
                                           (prelude:findl
                                            (:lambda (symbol) (libcore:eq symbol form))
                                            (prelude:%type-ref :bound lambda)))
                                        env))))
                                 (prelude:findl (:lambda (symbol) (libcore:eq symbol form)) bound-symbols))
                             acc)
                        acc)))
           ()
           body))
       (prelude:%type-ref :bound lambda))))

;;;
;;; %close-env = > closure
;;;
;;; libcore:frames => list of [*frame descriptor*] (function . vector)
;;; env => list of (symbol . lambda)
;;;
(libcore:intern :prelude "%close-env"
  (:lambda (closure env)
     ((:lambda (lambda fn env)
         (prelude:%make-funct lambda fn env))
      (prelude:%make-lambda
       (prelude:%lambda-prop :bound fn)
       (prelude:%lambda-prop :rest fn)
       (libcore:frames))
      (prelude:%funct-prop :fn fn))))

;;;
;;; compile closed lambda => function
;;;
(libcore:intern :prelude "%compile-function"
   (:lambda (form env)
      ((:lambda (lambda)
          ((:lambda (fn)
              (prelude:%make-funct lambda fn ()))
           (libcore:compile
            `(:lambda
              ,(prelude:%lambda-prop :bound lambda)
              ,(prelude:%compile-lambda-body lambda (libcore:nthcdr 2 form) env)))))
       (prelude:%compile-lambda-list (libcore:nth 1 form) env))))
