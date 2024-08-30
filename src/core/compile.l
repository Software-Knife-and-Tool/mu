;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core compiler
;;;

;;;
;;; special forms
;;;

;;;
;;; compile a core lambda form to a mu function
;;;
(mu:intern core "%compile-lambda"
   (:lambda (form env)
      (:if (mu:eq (mu:length form) 1)
           (core:raise form 'core:%compile-lambda "malformed lambda expression")
           ((:lambda (lambda body)
               ((:lambda (lambda-desc)
                  ((:lambda (compiled-function)
                     (:if (core:%orf
                           (core:%lambda-prop :lexenv lambda-desc)
                           (core:%lambda-prop :rest lambda-desc))
                          (core:%make-closure lambda-desc compiled-function ())
                          compiled-function))
                    (mu:compile
                     `(:lambda
                       ,(core:%lambda-prop :bound lambda-desc)
                       ,@(core:%compile-lambda-body
                          lambda-desc
                          body
                          env)))))
                (core:%compile-lambda-desc lambda env)))
            (mu:nth 1 form)
            (mu:nthcdr 2 form)))))

;;;
;;; compile a defmacro form to a bound macro symbol
;;;
(mu:intern core "%compile-defmacro"
   (:lambda (form env)
      ((:lambda (macro-symbol lambda-form)
          (:if (core:%symbolp macro-symbol)
               (core:%defmacro macro-symbol (core:%compile-lambda lambda-form env))
               (core:raise macro-symbol 'core:%compile-defmacro "not a symbol"))
          `(:quote ,macro-symbol))
       (mu:nth 1 form)
       (mu:nthcdr 1 form))))

;;;
;;; this if form can take two or three arguments
;;;
;;; returns an application of the %if form
;;;
(mu:intern core "%if"
   (:lambda (testf t-thunk f-thunk)
     (:if (core:null testf)
          (core:%apply f-thunk ())
          (core:%apply t-thunk ()))))

(mu:intern core "%compile-if"
   (:lambda (form env)
     ((:lambda (length)
          (:if (core:%orf (mu:eq length 3) (mu:eq length 4))
               ((:lambda (test t-arm f-arm)
                  (mu:compile `(,core:%if ,test ,t-arm ,f-arm)))
                (core:%compile (mu:nth 1 form) env)
                (core:%compile `(lambda () ,(mu:nth 2 form)) env)
                (core:%compile `(lambda () ,(mu:nth 3 form)) env))
               (core:raise form 'core:%compile-if "takes two or three arguments")))
       (mu:length form))))

;;;
;;; compile a quasiquote form
;;;
(mu:intern core "%compile-quasi"
   (:lambda (form env)
     (core:compile (core:%quasi-compile (mu:nth 1 form)))))               
   
;;;
;;; core compiler
;;;
;;; rewrites a function call/special call/macro call against an environment
;;; into something the runtime can execute.
;;;
;;; returns a mu form or %fn type (in the case of a define-core-lambda)
;;;
(mu:intern core "%compile"
   (:lambda (form env)
     (:if (core:%symbolp form)
          ((:lambda (ns name)
             (:if (mu:eq ns (mu:find-namespace ""))
                  ((:lambda (frame-ref)
                     ;;; (core:warn name "%compiling an unqualified symbol")
                     ;;; (core:warn env "in this environment")
                     form)
                   (core:%frame-ref form env))
                  form))
             (mu:symbol-namespace form)
             (mu:symbol-name form))
          (:if (core:%consp form)
               ((:lambda (function-form arg-list)
                  (:if (core:%keywordp function-form)
                       (mu:compile form)              ; mu special form (:key ...)
                       (:if (mu:eq :symbol (mu:type-of function-form)) ; special form or macro
                            ((:lambda (special)
                               (:if special
                                    (mu:apply (mu:symbol-value special) `(,form ,env))
                                    ((:lambda (macro-function)
                                       (:if macro-function
                                            (core:%compile-macro-call function-form arg-list env)
                                            (core:compile-funcall function-form arg-list env)))
                                     (core:macro-function function-form env))))
                             (mu:cdr (core:%assq
                                      function-form
                                      '((define-macro . core:%compile-defmacro)
                                        (if           . core:%compile-if)
                                        (%quasi%      . core:%compile-quasi)
                                        (lambda       . core:%compile-lambda)))))
                            (core:compile-funcall function-form arg-list env))))
                (mu:car form)
                (mu:cdr form))
           form))))

;;;
;;; core's compile interface, compiles in the null environment
;;;
(mu:intern core "compile"
   (:lambda (form)
     (core:%compile form ())))
