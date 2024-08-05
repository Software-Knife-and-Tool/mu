;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude compiler
;;;

;;;
;;; special forms
;;;

;;;
;;; compile a prelude lambda form to a mu function
;;;
(mu:intern prelude "%compile-lambda"
   (:lambda (form env)
      (:if (mu:eq (mu:length form) 1)
           (prelude:raise form 'prelude:%compile-lambda "malformed lambda expression")
           ((:lambda (lambda body)
               ((:lambda (lambda-desc)
                  ((:lambda (compiled-function)
                     (:if (prelude:%orf
                           (prelude:%lambda-prop :lexenv lambda-desc)
                           (prelude:%lambda-prop :rest lambda-desc))
                          (prelude:%make-fn lambda-desc compiled-function ())
                          compiled-function))
                    (mu:compile
                     `(:lambda
                       ,(prelude:%lambda-prop :bound lambda-desc)
                       ,@(prelude:%compile-lambda-body
                          lambda-desc
                          body
                          env)))))
                (prelude:%compile-lambda-list lambda env)))
            (mu:nth 1 form)
            (mu:nthcdr 2 form)))))

;;;
;;; compile a defmacro form to a bound macro symbol
;;;
(mu:intern prelude "%compile-defmacro"
   (:lambda (form env)
      ((:lambda (macro-symbol lambda-form)
          (:if (prelude:symbolp macro-symbol)
               (prelude:%defmacro macro-symbol (prelude:%compile-lambda lambda-form env))
               (prelude:raise macro-symbol 'prelude:%compile-defmacro "not a symbol"))
          `(:quote ,macro-symbol))
       (mu:nth 1 form)
       (mu:nthcdr 1 form))))

;;;
;;; this if form can take two or three arguments
;;;
;;; returns an application of the %if form
;;;
(mu:intern prelude "%if"
   (:lambda (testf t-thunk f-thunk)
     (:if (prelude:null testf)
          (prelude:apply f-thunk ())
          (prelude:apply t-thunk ()))))

(mu:intern prelude "%compile-if"
   (:lambda (form env)
     ((:lambda (length)
          (:if (prelude:%orf (mu:eq length 3) (mu:eq length 4))
               ((:lambda (test t-arm f-arm)
                  (mu:compile `(,prelude:%if ,test ,t-arm ,f-arm)))
                (prelude:%compile (mu:nth 1 form) env)
                (prelude:%compile `(lambda () ,(mu:nth 2 form)) env)
                (prelude:%compile `(lambda () ,(mu:nth 3 form)) env))
               (prelude:raise form 'prelude:%compile-if "takes two or three arguments")))
       (mu:length form))))

;;;
;;; compile a quasiquote form
;;;
(mu:intern prelude "%compile-quasi"
   (:lambda (form env)
     (prelude:compile (prelude:%quasi-compile (mu:nth 1 form)))))               
   
;;;
;;; prelude compiler
;;;
;;; rewrites a function call/special call/macro call against an environment
;;; into something the runtime can execute.
;;;
;;; returns a mu form or %fn type (in the case of a define-prelude-lambda)
;;;
(mu:intern prelude "%compile"
   (:lambda (form env)
     (:if (prelude:symbolp form)
          ((:lambda (ns name)
             (:if (mu:eq ns (mu:find-namespace ""))
                  ((:lambda (frame-ref)
                     ;;; (prelude:warn name "%compiling an unqualified symbol")
                     ;;; (prelude:warn env "in this environment")
                     form)
                   (prelude:%frame-ref form env))
                  form))
             (prelude:symbol-namespace form)
             (prelude:symbol-name form))
          (:if (prelude:consp form)
               ((:lambda (function-form arg-list)
                  (:if (prelude:keywordp function-form)
                       (mu:compile form)              ; mu special form (:key ...)
                       (:if (mu:eq :symbol (mu:type-of function-form)) ; special form or macro
                            ((:lambda (special)
                               (:if special
                                    (mu:apply (mu:symbol-value special) `(,form ,env))
                                    ((:lambda (macro-function)
                                       (:if macro-function
                                            (prelude:%compile-macro-call function-form arg-list env)
                                            (prelude:%compile-funcall function-form arg-list env)))
                                     (prelude:macro-function function-form env))))
                             (mu:cdr (prelude:%assq
                                      function-form
                                      '((define-macro . prelude:%compile-defmacro)
                                        (if           . prelude:%compile-if)
                                        (%quasi%      . prelude:%compile-quasi)
                                        (lambda       . prelude:%compile-lambda)))))
                            (prelude:%compile-funcall function-form arg-list env))))
                (mu:car form)
                (mu:cdr form))
           form))))

;;;
;;; prelude's compile interface, compiles in the null environment
;;;
(mu:intern prelude "compile"
   (:lambda (form)
     (prelude:%compile form ())))
