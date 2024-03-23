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
(core:intern :prelude "%compile-lambda"
   (:lambda (form env)
      (:if (core:eq (core:length form) 1)
           (prelude:raise form 'prelude:%compile-lambda "malformed lambda expression")
           ((:lambda (lambda body)
               ((:lambda (lambda-desc)
                   ((:lambda (compiled-function)
                       (:if (prelude:%orf
                             (prelude:%lambda-prop :lexenv lambda-desc)
                             (prelude:%lambda-prop :rest lambda-desc))
                            (prelude:%make-funct lambda-desc compiled-function ())
                            compiled-function))
                    (core:compile
                     `(:lambda
                       ,(prelude:%lambda-prop :bound lambda-desc)
                       ,@(prelude:%compile-lambda-body
                          lambda-desc
                          body
                          env)))))
                (prelude:%compile-lambda-list lambda env)))
            (core:nth 1 form)
            (core:nthcdr 2 form)))))

;;;
;;; compile a defmacro form to a bound macro symbol
;;;
;;; returns the macro symbol
;;;
(core:intern :prelude "%compile-defmacro"
   (:lambda (form env)
      ((:lambda (macro-symbol lambda-form)
          (:if (prelude:symbolp macro-symbol)
               (prelude:%defmacro macro-symbol (prelude:%compile-lambda lambda-form env))
               (prelude:raise macro-symbol 'prelude:%compile-defmacro "not a symbol"))
          `(:quote ,macro-symbol))
       (core:nth 1 form)
       (core:nthcdr 1 form))))

;;;
;;; this if form can take two or three arguments
;;;
;;; returns an application of the if form
;;;
(core:intern :prelude "%make-thunk"
  (:lambda (form env)
    (prelude:%compile
     `(:lambda () ,form)
     env)))

(core:intern :prelude "%compile-if"
   (:lambda (form env)
      ((:lambda (length)
          (:if (prelude:%orf (core:eq length 3) (core:eq length 4))
               ((:lambda (t-arm f-arm)
                  (core:compile `(:if ,(core:nth 1 form) ,t-arm ,f-arm)))
                (:if (prelude:constantp (core:nth 2 form))
                     (core:nth 2 form)
                     `(prelude:apply ,(prelude:%make-thunk (core:nth 2 form) env) ()))
                (:if (prelude:constantp (core:nth 3 form))
                     (core:nth 3 form)
                     `(prelude:apply ,(prelude:%make-thunk (core:nth 3 form) env) ())))
               (prelude:raise form 'prelude:%compile-if "takes two or three arguments")))
       (core:length form))))

;;;
;;; prelude compiler
;;;
;;; rewrites a function call/special call/macro call against an environment
;;; into something the runtime can execute. constants compile to themselves.
;;;
;;; returns a mu form or funct type (in the case of a define-prelude-lambda)
;;;
(core:intern :prelude "%compile"
   (:lambda (form env)
      (:if (prelude:consp form)
           ((:lambda (function-form arg-list)
               (:if (prelude:keywordp function-form)
                    (core:compile form)                    ; mu special form (:key ...)
                    (:if (core:eq :symbol (core:type-of function-form)) ; special form or macro
                         ((:lambda (special)
                             (:if special
                                  (core:apply (core:sy-val special) `(,form ,env))
                                  ((:lambda (macro-function)
                                      (:if macro-function
                                           (prelude:%compile-macro-call function-form arg-list env)
                                           (prelude:%compile-funcall function-form arg-list env)))
                                   (prelude:macro-function function-form env))))
                          (core:cdr (prelude:%assoc
                                   function-form
                                   '((define-macro . prelude:%compile-defmacro)
                                     (if           . prelude:%compile-if)
                                     (lambda       . prelude:%compile-lambda)))))
                    (prelude:%compile-funcall function-form arg-list env))))
            (core:car form)
            (core:cdr form))
           form)))

;;;
;;; the external compile interface, compiles in
;;; the null environment
;;;
(core:intern :prelude "compile"
   (:lambda (form)
     (prelude:%compile form ())))
