;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude compiler
;;;

;;;
;;; special forms
;;;

;;;
;;; compile a lambda form to a function prelude type
;;;
;;; returns the compiled function
;;;
(crux:intern prelude "%compile-lambda"
   (:lambda (form env)
      (:if (crux:eq (crux:length form) 1)
           (prelude:raise form 'prelude:%compile-lambda "malformed lambda expression")
           ((:lambda (lambda body)
               ((:lambda (lambda-desc)
                   ((:lambda (compiled-function)
                       (:if (prelude:%orf
                             (prelude:%lambda-prop :lexenv lambda-desc)
                             (prelude:%lambda-prop :rest lambda-desc))
                            (prelude:%make-fn lambda-desc compiled-function ())
                            compiled-function))
                    (crux:compile
                     `(:lambda
                       ,(prelude:%lambda-prop :bound lambda-desc)
                       ,@(prelude:%compile-lambda-body
                          lambda-desc
                          body
                          env)))))
                (prelude:%compile-lambda-list lambda env)))
            (crux:nth 1 form)
            (crux:nthcdr 2 form)))))

;;;
;;; compile a defmacro form to a bound macro symbol
;;;
;;; returns the macro symbol
;;;
(crux:intern prelude "%compile-defmacro"
   (:lambda (form env)
      ((:lambda (macro-symbol lambda-form)
          (:if (prelude:symbolp macro-symbol)
               (prelude:%defmacro macro-symbol (prelude:%compile-lambda lambda-form env))
               (prelude:raise macro-symbol 'prelude:%compile-defmacro "not a symbol"))
          `(:quote ,macro-symbol))
       (crux:nth 1 form)
       (crux:nthcdr 1 form))))

;;;
;;; this if form can take two or three arguments
;;;
;;; returns an application of the if form
;;;
(crux:intern prelude "%if"
   (:lambda (testf t-thunk f-thunk)
     (:if (prelude:apply testf ())
          (prelude:apply t-thunk ())
          (prelude:apply f-thunk ()))))

(crux:intern prelude "%compile-if"
   (:lambda (form env)
      ((:lambda (length)
          (:if (prelude:%orf (crux:eq length 3) (crux:eq length 4))
               ((:lambda (test t-arm f-arm)
                  (crux:compile `(prelude:%if ,test ,t-arm ,f-arm)))
                (prelude:%compile `(lambda () ,(crux:nth 1 form)) env)
                (prelude:%compile `(lambda () ,(crux:nth 2 form)) env)
                (prelude:%compile `(lambda () ,(crux:nth 3 form)) env))
               (prelude:raise form 'prelude:%compile-if "takes two or three arguments")))
       (crux:length form))))

;;;
;;; compile a quasiquote form
;;;
(crux:intern prelude "%compile-quasi"
   (:lambda (form env)
     (prelude:compile (prelude:%quasi-compile (crux:nth 1 form)))))               
   
;;;
;;; prelude compiler
;;;
;;; rewrites a function call/special call/macro call against an environment
;;; into something the runtime can execute.
;;;
;;; returns a lib form or %fn type (in the case of a define-prelude-lambda)
;;;
(crux:intern prelude "%compile"
   (:lambda (form env)
      (:if (prelude:consp form)
           ((:lambda (function-form arg-list)
               (:if (prelude:keywordp function-form)
                    (crux:compile form)                    ; lib special form (:key ...)
                    (:if (crux:eq :symbol (crux:type-of function-form)) ; special form or macro
                         ((:lambda (special)
                             (:if special
                                  (crux:apply (crux:symbol-value special) `(,form ,env))
                                  ((:lambda (macro-function)
                                      (:if macro-function
                                           (prelude:%compile-macro-call function-form arg-list env)
                                           (prelude:%compile-funcall function-form arg-list env)))
                                   (prelude:macro-function function-form env))))
                          (crux:cdr (prelude:%assq
                                   function-form
                                   '((define-macro . prelude:%compile-defmacro)
                                     (if           . prelude:%compile-if)
                                     (%quasi%      . prelude:%compile-quasi)
                                     (lambda       . prelude:%compile-lambda)))))
                    (prelude:%compile-funcall function-form arg-list env))))
            (crux:car form)
            (crux:cdr form))
           form)))

;;;
;;; repl's compile interface, compiles in the null environment
;;;
(crux:intern prelude "compile"
   (:lambda (form)
     (prelude:%compile form ())))
