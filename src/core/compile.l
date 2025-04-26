;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core compiler
;;;

;;;
;;; special forms
;;;

;;;
;;; compile a lambda form to a closure
;;;
(mu:intern core "%special-lambda"
   (:lambda (form env)
     (:if (mu:eq 0 (mu:length form))
          (core:%raise form 'core:%special-lambda "malformed lambda expression")
          (core:%compile-closure (mu:nth 1 form) (mu:nthcdr 2 form) env))))

;;;
;;; compile a defmacro form to a bound macro symbol
;;;
(mu:intern core "%special-macro"
   (:lambda (form env)
     (:if (mu:eq 1 (mu:length form))
          (core:%raise form 'core:%compile-macro-call "malformed lambda expression")
          ((:lambda (symbol)
             (:if (core:symbolp symbol)
                  (core:%defmacro symbol (core:%compile-closure (mu:nth 2 form) (mu:nthcdr 3 form) env))
                  (core:%raise symbol 'core:%special-macro "not a symbol"))
             (mu:cons :quote symbol))
           (mu:nth 1 form)))))

;;;
;;; if can take two or three arguments
;;;
(mu:intern core "%nilf"
   (:lambda ()))

(mu:intern core "%special-if"
   (:lambda (form env)
     ((:lambda (length)
        (:if (core:%or (mu:eq 3 length) (mu:eq 4 length))
             ((:lambda (test t-arm f-arm)
                `(mu:%if ,test ,t-arm ,f-arm))
              (core:%compile (mu:nth 1 form) env)
              (mu:compile `(:lambda () ,(core:%compile (mu:nth 2 form) env)))
              (:if (mu:eq 3 length)
                   core:%nilf
                   (mu:compile `(:lambda () ,(core:%compile (mu:nth 3 form) env)))))
        (core:%raise form 'core:%special-if "takes two or three arguments")))
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
(mu:intern core "%frame-map" (mu:open :string :bidir "" :t))

(mu:intern core "%add-frame"
   (:lambda (name func)
     (mu:write #\space () core:%frame-map)
     (mu:write (mu:cons name (mu:repr func)) () core:%frame-map)))

(mu:intern core "%frame-ref"
   (:lambda (binding index offset)
     ((:lambda (name)
        ((:lambda (frames)
           (mu:%frame-ref frames offset))
         (mu:symbol-value (mu:intern core:*closures/* name ()))))
        (mu:symbol-name (core:%env-prop :name binding)))))

(mu:intern core "%resolve-lexical"
   (:lambda (symbol env)          
     (:if (mu:eq mu:*null/* (mu:symbol-namespace symbol))
          ((:lambda (binding)
             (:if (core:null binding)
                  symbol
                  ((:lambda (tag index)
                     `(core:%frame-ref ,binding ,tag ,index))
                   (core:%env-prop :frame binding)
                   (core:%position-if
                    (:lambda (sym)
                      (mu:eq sym symbol))
                    (core:%env-prop :symbols binding)))))
           (core:%find-if
            (:lambda (binding)
              (core:%find-if
               (:lambda (sym)
                 (mu:eq sym symbol))
               (core:%env-prop :symbols binding)))
            env))
          symbol)))

;;;
;;; compile function applications
;;;

;;;
;;; argument lists
;;;
(mu:intern core "%arg-list"
   (:lambda (arg-list)
      (core:%foldr
       (:lambda (elt acc)
         `(mu:cons ,elt ,acc))
       ()
       arg-list)))

(mu:intern core "%quoted-arg-list"
   (:lambda (arg-list)
      (core:%foldr
       (:lambda (elt acc)
         `(mu:cons ,(core:%quote elt) ,acc))
       ()
       arg-list)))

(mu:intern core "%lambda-arg-list"
   (:lambda (function arg-list)
     (:if (core:%closurep function)
          ((:lambda (arity rest nreqs)
             (:if rest
                  (:if (mu:eq 0 arity)
                       `(mu:cons ,(core:%arg-list arg-list) ())
                       ((:lambda (req-list rest-list)
                          (:if (core:%and (core:null rest-list) (core:null req-list))
                               ()
                               (core:%arg-list `(,@req-list ,(core:%arg-list rest-list)))))
                        (core:%dropr arg-list (mu:sub (mu:length arg-list) arity))
                        (core:%dropl arg-list arity)))
                  (core:%arg-list arg-list)))
           (core:%closure-prop :arity function)
           (core:%closure-prop :rest function)
           (mu:length (core:%closure-prop :require function)))
          (core:%arg-list arg-list))))

(mu:intern core "%quoted-lambda-arg-list"
   (:lambda (function arg-list)
     (:if (core:%closurep function)
          ((:lambda (arity rest nreqs)
              (:if rest
                   (:if (mu:eq 0 arity)
                        `(mu:cons ,(core:%quoted-arg-list arg-list) ())
                        ((:lambda (req-list rest-list)
                           (:if (core:%and (core:null rest-list) (core:null req-list))
                                ()
                                (core:%quoted-arg-list `(,@req-list ,rest-list))))
                         (core:%dropr arg-list (mu:sub (mu:length arg-list) arity))
                         (core:%dropl arg-list arity)))
                   (core:%quoted-arg-list arg-list)))
            (core:%closure-prop :arity function)
            (core:%closure-prop :rest function)
            (mu:length (core:%closure-prop :require function)))
           (core:%quoted-arg-list arg-list))))

;;;
;;; compile argument lists
;;;
(mu:intern core "%compile-arg-list"
   (:lambda (arg-list env)
      (core:%foldr
       (:lambda (elt acc)
          `(mu:cons ,@(mu:cons elt `(,acc))))
       ()
       (core:%mapcar
        (:lambda (form)
           (core:%compile form env))
        arg-list))))

(mu:intern core "%compile-lambda-arg-list"
   (:lambda (function arg-list env)
     (core:%lambda-arg-list function (core:%mapcar (:lambda (elt) (core:%compile elt env)) arg-list))))

(mu:intern core "%compile-quoted-lambda-arg-list"
   (:lambda (function arg-list env)
     (core:%quoted-lambda-arg-list function arg-list)))
     
;;;
;;; compile function application to mu form
;;;
(mu:intern core "%compile-funcall"
   (:lambda (function-form arg-list env)
     (:if (core:%genericp function-form)
          `(core:%apply ,function ,(core:%compile-arg-list arg-list env))
          (:if (core:consp function-form)
               ((:lambda (function)
                  (:if (core:%or (core:functionp function) (core:consp function))
                       (:if (core:functionp function)
                            (:if (core:%closurep function)
                                 `(,core:%apply
                                   ,function
                                   ,(core:%closure-prop :mu function)
                                   ,(core:%compile-lambda-arg-list function arg-list env))
                                 `(,mu:apply ,function ,(core:%compile-arg-list arg-list env)))
                            `(mu:apply ,function ,(core:%compile-arg-list arg-list env)))
                       (core:%raise function 'core:%compile-funcall "not a function designator")))
                (core:%compile function-form env))
               (:if (mu:boundp function-form)
                    ((:lambda (function)
                       (:if (core:%genericp function)
                            `(core:%apply-generic ,function ,(core:%compile-arg-list arg-list env))
                            (:if (core:functionp function)
                                 (:if (core:%closurep function)
                                      `(,core:%apply
                                        ,function
                                        ,(core:%closure-prop :mu function)
                                        ,(core:%compile-lambda-arg-list function arg-list env))
                                      `(mu:apply ,function ,(core:%compile-arg-list arg-list env)))
                                 (core:%raise function 'core:%compile-funcall "not a function designator"))))
                     (mu:symbol-value function-form))
                    `(,core:apply ,function-form ,(core:%compile-arg-list arg-list env)))))))

(mu:intern core "%compile"
   (:lambda (form env)
     (:if (mu:eq :symbol (mu:type-of form))
          (core:%resolve-lexical form env)
          (:if (core:consp form)
               ((:lambda (function-form arg-list)
                  (:if (core:keywordp function-form)
                       form                                            ; mu special form (:key ...)
                       (:if (mu:eq :symbol (mu:type-of function-form)) ; special form/function/macro call
                            ((:lambda (special)
                               (:if special
                                    (mu:apply (mu:symbol-value special) (core:%list2 form env))
                                    ((:lambda (macro-function)
                                       (:if macro-function
                                            (core:%compile-macro-call function-form arg-list env)
                                            (core:%compile-funcall function-form arg-list env)))
                                     (core:macro-function function-form env))))
                             (mu:cdr
                              (core:%assq
                               function-form
                               '((%defmacro . core:%special-macro)
                                 (%if       . core:%special-if)
                                 (%quasi%   . core:%special-quasi)
                                 (%lambda   . core:%special-lambda)))))
                            (:if (core:consp function-form)
                                 (core:%compile-funcall function-form arg-list env)
                                 (core:%raise form 'core:%compile "malformed lambda expression")))))
                (mu:car form)
                (mu:cdr form))
               form))))

(mu:intern core "compile"
   (:lambda (form)
     (mu:write #\( () core:%frame-map)
     ((:lambda (mu-form)
        (mu:compile mu-form))
      ((:lambda (form)
         (mu:write #\) () core:%frame-map)
         ((:lambda (frame-map)
            (core:%mapc
             (:lambda (frame-ref)
               (mu:intern
                core:*closures/*
                (mu:symbol-name (mu:car frame-ref))
                (mu:unrepr (mu:cdr frame-ref))))
             frame-map)
            form)
          (mu:read core:%frame-map () ())))
       (core:%compile form ())))))
