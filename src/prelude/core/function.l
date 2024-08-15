;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; closures
;;;

;;;
;;; function properties
;;;
(mu:intern prelude "%fn-prop"
   (:lambda (prop fn)
      (:if (prelude:findl (:lambda (fn-prop) (mu:eq prop fn-prop)) '(:lambda :arity :fn :env))
           ((:lambda (ref)
               ;;; (prelude:raise-when prelude:null prop 'prelude:%function-prop "property not bound")
               (mu:cdr ref))
            (prelude:%type-ref prop fn))
           (prelude:%lambda-prop prop (prelude:%fn-prop :lambda fn)))))

;;;
;;; predicates
;;;
(mu:intern prelude "%prelude-function-p"
   (:lambda (fn)
     (prelude:%andf
      (prelude:%prelude-type-p fn)
      (mu:eq '%fn (prelude:type-of fn)))))

(mu:intern prelude "functionp"
   (:lambda (fn)
      (:if (mu:eq :func (mu:type-of fn))
           :t
           (prelude:%prelude-function-p fn))))

;;;
;;; print prelude function
;;;
(mu:intern prelude "%describe-fn"
   (:lambda (fn)
     (:if (prelude:%prelude-function-p fn)
          ((:lambda (lambda arity fn env)
             (prelude:eprint "" "%describe-fn:prelude")
             (prelude:eprint lambda "    lambda")
             (prelude:eprint arity "    arity")
             (prelude:eprint fn "    fn")
             (prelude:eprint env "    env"))
           (prelude:%fn-prop :lambda fn)
           (prelude:%fn-prop :arity fn)
           (prelude:%fn-prop :fn fn)
           (prelude:%fn-prop :env fn))
          (prelude:eprint (mu:view fn) "%describe-fn:mu-fn"))))

;;;
;;; [*function descriptor*] #s(:<type> :%fn (lambda nargs fn env))
;;;
;;; lambda: lambda type
;;; nargs:  number of reqs plus rest
;;; fn:     mu function implementation
;;; env:    dynamic environment, list of frames
;;;
(mu:intern prelude "%make-fn"
   (:lambda (lambda fn env)
      ((:lambda (nargs)
          (prelude:%make-type "%fn"
             `(,(mu:cons :lambda lambda)
               ,(mu:cons :arity  nargs)
               ,(mu:cons :fn     fn)
               ,(mu:cons :env    env))))
       (mu:difference
        (mu:length (prelude:%lambda-prop :bound lambda))
        (:if (prelude:%lambda-prop :rest lambda) 1 0)))))

;;;
;;; create closed environment
;;;
(mu:intern prelude "%closure-env"
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
;;; compile closed lambda => mu function
;;;
(mu:intern prelude "%compile-function"
   (:lambda (form env)
      ((:lambda (lambda)
          ((:lambda (fn)
              (prelude:%make-fn lambda fn ()))
           (mu:compile
            `(:lambda
              ,(prelude:%lambda-prop :bound lambda)
              ,(prelude:%compile-lambda-body lambda (mu:nthcdr 2 form) env)))))
       (prelude:%compile-lambda-desc (mu:nth 1 form) env))))
