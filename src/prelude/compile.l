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
(libcore:intern :prelude "%compile-lambda"
   (:lambda (form env)
      (:if (libcore:eq (libcore:length form) 1)
           (prelude:raise form 'prelude:%compile-lambda "malformed lambda expression")
           ((:lambda (lambda body)
               ((:lambda (lambda-desc)
                   ((:lambda (compiled-function)
                       (:if (prelude:%orf
                             (prelude:%lambda-prop :lexenv lambda-desc)
                             (prelude:%lambda-prop :rest lambda-desc))
                            (prelude:%make-funct lambda-desc compiled-function ())
                            compiled-function))
                    (libcore:compile
                     `(:lambda
                       ,(prelude:%lambda-prop :bound lambda-desc)
                       ,@(prelude:%compile-lambda-body
                          lambda-desc
                          body
                          env)))))
                (prelude:%compile-lambda-list lambda env)))
            (libcore:nth 1 form)
            (libcore:nthcdr 2 form)))))

;;;
;;; compile a defmacro form to a bound macro symbol
;;;
;;; returns the macro symbol
;;;
(libcore:intern :prelude "%compile-defmacro"
   (:lambda (form env)
      ((:lambda (macro-symbol lambda-form)
          (:if (prelude:symbolp macro-symbol)
               (prelude:%defmacro macro-symbol (prelude:%compile-lambda lambda-form env))
               (prelude:raise macro-symbol 'prelude:%compile-defmacro "not a symbol"))
          `(:quote ,macro-symbol))
       (libcore:nth 1 form)
       (libcore:nthcdr 1 form))))

;;;
;;; this if form can take two or three arguments
;;;
;;; returns an application of the if form
;;;
(libcore:intern :prelude "%make-thunk"
  (:lambda (form env)
    (prelude:%compile
     `(:lambda () ,form)
     env)))

(libcore:intern :prelude "%compile-if"
   (:lambda (form env)
      ((:lambda (length)
          (:if (prelude:%orf (libcore:eq length 3) (libcore:eq length 4))
               ((:lambda (t-arm f-arm)
                  (libcore:compile `(:if ,(libcore:nth 1 form) ,t-arm ,f-arm)))
                (:if (prelude:constantp (libcore:nth 2 form))
                     (libcore:nth 2 form)
                     `(prelude:apply ,(prelude:%make-thunk (libcore:nth 2 form) env) ()))
                (:if (prelude:constantp (libcore:nth 3 form))
                     (libcore:nth 3 form)
                     `(prelude:apply ,(prelude:%make-thunk (libcore:nth 3 form) env) ())))
               (prelude:raise form 'prelude:%compile-if "takes two or three arguments")))
       (libcore:length form))))

;;;
;;; compile a quasiquote form
;;;
(libcore:intern :prelude "%compile-quasi"
   (:lambda (form env)
     (prelude:compile (prelude:%qq-compile (libcore:nth 1 form)))))               
   
;;;
;;; prelude compiler
;;;
;;; rewrites a function call/special call/macro call against an environment
;;; into something the runtime can execute. constants compile to themselves.
;;;
;;; returns a mu form or funct type (in the case of a define-prelude-lambda)
;;;
(libcore:intern :prelude "%compile"
   (:lambda (form env)
      (:if (prelude:consp form)
           ((:lambda (function-form arg-list)
               (:if (prelude:keywordp function-form)
                    (libcore:compile form)                    ; mu special form (:key ...)
                    (:if (libcore:eq :symbol (libcore:type-of function-form)) ; special form or macro
                         ((:lambda (special)
                             (:if special
                                  (libcore:apply (libcore:sy-val special) `(,form ,env))
                                  ((:lambda (macro-function)
                                      (:if macro-function
                                           (prelude:%compile-macro-call function-form arg-list env)
                                           (prelude:%compile-funcall function-form arg-list env)))
                                   (prelude:macro-function function-form env))))
                          (libcore:cdr (prelude:%assoc
                                   function-form
                                   '((define-macro . prelude:%compile-defmacro)
                                     (if           . prelude:%compile-if)
                                     (%quasi%      . prelude:%compile-quasi)
                                     (lambda       . prelude:%compile-lambda)))))
                    (prelude:%compile-funcall function-form arg-list env))))
            (libcore:car form)
            (libcore:cdr form))
           form)))

;;;
;;; the external compile interface, compiles in
;;; the null environment
;;;
(libcore:intern :prelude "compile"
   (:lambda (form)
     (prelude:%compile form ())))
