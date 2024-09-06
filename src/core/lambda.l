;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lambda
;;;
(mu:intern core "%lambdap"
    (:lambda (fn)
      (core:%andf
       (core:%core-type-p fn)
       (mu:eq '%lambda (core:type-of fn)))))

;;;
;;; implementation
;;;
(core:def-type "%lambda"
    '((:bound  . :listp)
      (:frame  . :func)
      (:rest   . :symbolp)
      (:lexenv . :listp)))

(mu:intern core "%lambda-prop"
   (:lambda (prop lambda)
      (:if (core:%findl (:lambda (lambda-prop) (mu:eq prop lambda-prop)) '(:bound :frame :rest :lexenv))
           ((:lambda (ref)
              (mu:cdr ref))
            (core:%type-ref prop lambda))
           (core:raise prop 'core:%lambda-prop "not a lambda property"))))

(mu:intern core "%apply-lambda%" (:lambda (fn) (fn)))
(mu:intern core "%make-lambda"
   (:lambda (lambda-list rest-sym env)
     ((:lambda (frame)
        (core:%make-type "%lambda"
           `(,(mu:cons :bound  lambda-list)
             ,(mu:cons :frame  frame)
             ,(mu:cons :rest   rest-sym)
             ,(mu:cons :lexenv env))))
      (:if (core:null lambda-list)
           core:%apply-lambda%
           ((:lambda (fn-symbol arg-list)
              (mu:compile `(:lambda ,(mu:cons fn-symbol lambda-list) (mu:apply fn-symbol arg-list))))
            (mu:make-symbol "fn")
            (core:%arg-list lambda-list))))))

;;;
;;; compile
;;;
(mu:intern core "%compile-lambda-desc"
   (:lambda (lambda env)
     (:if (core:%findl (:lambda (el) (core:null (mu:eq :symbol (mu:type-of el)))) lambda)
          (core:raise lambda 'core:%compile-lambda-desc "list syntax")
          ((:lambda (desc)
             (:if (core:%fixnump desc)
                  (core:%make-lambda lambda () env)
                  desc))
           (core:%foldl
            (:lambda (el acc)
              (:if (core:%lambdap acc)
                   acc
                   (:if (mu:eq '&rest el)
                        (:if (mu:eq (mu:length lambda) (mu:add 2 acc))
                             (core:%make-lambda
                              `(,@(core:%dropr lambda 2) ,@(core:%dropl lambda (mu:sub (mu:length lambda) 1)))
                              (mu:nth (mu:sub (mu:length lambda) 1) lambda)
                              env)
                             (core:raise lambda 'core:%compile-lambda-desc "rest botch"))
                        (mu:add 1 acc))))
            0
            lambda)))))

(mu:intern core "%compile-lambda-body"
   (:lambda (lambda-desc body env)
      (:if (core:null body)
           ()
           ((:lambda (env)
               (core:%maplist
                (:lambda (body-cdr)
                   (:if (mu:cdr body-cdr)
                        ((:lambda (value)
                            (core:%compile value env))
                            (mu:car body-cdr))
                        ((:lambda (form)
                            (:if (core:%consp form)
                                 (:if (mu:eq 'lambda (mu:car form))
                                      (core:%compile-function form env)
                                      (core:%compile form env))
                                 (core:%compile form env)))
                         (mu:car body-cdr))))
                body))
          (mu:cons lambda-desc env)))))

(mu:intern core "%frame-ref"
   (:lambda (symbol env)
    (core:%foldl
     (:lambda (frame acc)
       (:if acc
            acc
            ((:lambda (offset)
               (:if offset
                    (mu:compile
                     `(mu:%frame-ref
                       ,(core:%lambda-prop :frame frame)
                       ,offset))
                    ()))
            (core:%positionl
             (:lambda (el) (mu:eq el symbol))
             (core:%lambda-prop :bound frame)))))
     ()
     env)))
