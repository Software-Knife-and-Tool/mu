;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core compiler
;;;

;;;
;;; special forms
;;;

;;;
;;; compile a lambda form to a core-function
;;;
(mu:intern core "%special-lambda"
   (:lambda (form env)
     (:if (mu:eq 0 (mu:length form))
          (core:raise form 'core:%special-lambda "malformed lambda expression")
          (core:compile-core-function (mu:nth 1 form) (mu:nthcdr 2 form) env))))

;;;
;;; compile a defmacro form to a bound macro symbol
;;;
(mu:intern core "%special-macro"
   (:lambda (form env)
     (:if (mu:eq 1 (mu:length form))
          (core:raise form 'core:%compile-macro "malformed lambda expression")
          ((:lambda (symbol)
             (:if (core:%symbolp symbol)
                  (core:%defmacro symbol (core:compile-core-function (mu:nth 2 form) (mu:nthcdr 3 form) env))
                  (core:raise symbol 'core:%special-macro "not a symbol"))
             (mu:cons :quote symbol))
           (mu:nth 1 form)))))

;;;
;;; if can take two or three arguments
;;;
(mu:intern core "%special-if"
   (:lambda (form env)
     ((:lambda (length)
          (:if (core:logor (mu:eq length 3) (mu:eq length 4))
               ((:lambda (test t-arm f-arm)
                  ((:lambda (form)
                     (mu:cons 'core:apply (mu:cons form (mu:cons () ()))))
                   (mu:cons :if (mu:cons test (mu:cons t-arm (mu:cons f-arm ()))))))
                (core:%compile (mu:nth 1 form) env)
                (core:%compile (mu:cons 'lambda (mu:cons () (mu:cons (mu:nth 2 form) ()))) env)
                (core:%compile (mu:cons 'lambda (mu:cons () (mu:cons (mu:nth 3 form) ()))) env))
               (core:raise form 'core:%special-if "takes two or three arguments")))
      (mu:length form))))

;;;
;;; compile a quasiquote form
;;;
(mu:intern core "%special-quasi"
   (:lambda (form env)
     (core:%compile (core:%quasi-compile (mu:nth 1 form)) env)))               

;;;
;;; core compiler
;;;
(mu:intern core "%compile"
   (:lambda (form env)
     (:if (core:%symbolp form)
          form
          (:if (core:%consp form)
               ((:lambda (function-form arg-list)
                  (:if (core:%keywordp function-form)
                       form                                            ; mu special form (:key ...)
                       (:if (mu:eq :symbol (mu:type-of function-form)) ; special form/function/macro call
                            ((:lambda (special)
                               (:if special
                                    (mu:apply (mu:symbol-value special) (core:%list2 form env))
                                    ((:lambda (macro-function)
                                       (:if macro-function
                                            (core:%compile-macro-call function-form arg-list env)
                                            (core:compile-funcall function-form arg-list env)))
                                     (core:%macro-function function-form env))))
                             (mu:cdr
                              (core:%assq
                               function-form
                               '((%defmacro . core:%special-macro)
                                 (if        . core:%special-if)
                                 (%quasi%   . core:%special-quasi)
                                 (lambda    . core:%special-lambda)))))
                            (:if (core:%consp function-form)
                                 (core:compile-funcall function-form arg-list env)
                                 (core:raise form 'core:%compile "malformed lambda expression")))))
                (mu:car form)
                (mu:cdr form))
               form))))

(mu:intern core "compile"
   (:lambda (form)
     (mu:compile (core:%compile form ()))))          
