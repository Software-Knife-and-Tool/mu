;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; exception deftype
;;;

;;;
;;; cond:   condition type symbol
;;; value:  exception raised on value
;;; source: function designator, usually a symbol
;;; reason: extended condition, usually a string
;;; env:    dynamic environment, list of frames
;;;
(core:%deftype "%exception"
    '((:cond   . :symbolp)
      (:value  . :t)
      (:source . :symbolp)
      (:reason . :string)
      (:env    . :listp)))

(mu:intern core "%exception-prop"
   (:lambda (prop exception)
     (core:%type-prop prop exception)))

(mu:intern core "%make-exception"
   (:lambda (cond value source reason env)
      (core:%make-type "%exception"
        `(,(mu:cons :cond   cond)
          ,(mu:cons :value  value)
          ,(mu:cons :source source)
          ,(mu:cons :reason reason)
          ,(mu:cons :env    env)))))

;;;
;;; accessors
;;;
(mu:intern core "exception-cond"
   (:lambda (exception)
     (:if (core:exceptionp exception)
          (core:%exception-prop :cond exception)
          (core:%raise exception 'core:exception-code "is not an exception"))))

(mu:intern core "exception-value"
   (:lambda (exception)
     (:if (core:exceptionp exception)
          (core:%exception-prop :value exception)
          (core:%raise exception 'core:exception-value "is not an exception"))))

(mu:intern core "exception-source"
   (:lambda (exception)
     (:if (core:exceptionp exception)
          (core:%exception-prop :source exception)
          (core:%raise exception 'core:exception-sopurce "is not an exception"))))

(mu:intern core "exception-reason"
   (:lambda (exception)
     (:if (core:exceptionp exception)
          (core:%exception-prop :reason exception)
          (core:%raise exception 'core:exception-reason "is not an exception"))))

(mu:intern core "exception-env"
   (:lambda (exception)
     (:if (core:exceptionp exception)
          (core:%exception-prop :env exception)
          (core:%raise exception 'core:exception-env "is not an exception"))))

;;;
;;; predicate
;;;
(mu:intern core "exceptionp"
    (:lambda (ex)
      (core:%and
       (core:%typep ex)
       (mu:eq '%exception (core:type-of ex)))))

;;;
;;; functions
;;;
(mu:intern core "%exceptionf"
   (:lambda (stream format-string printenv exception)
     ((:lambda (cond value source reason env)
         (:if (core:null format-string)
              (core:%format stream "~S ~S ~S ~S~%" `(,cond ,value ,source ,reason))
              (core:%format stream format-string `(,cond ,value ,source ,reason)))
         (:if (core:%and printenv env)
              (core:%prog2
                  (core:%format stream "env:~%" ())
                  (core:%mapc
                   (:lambda (frame)
                      (core:%format stream "  ~A~T(~A ~A)~%"
                                   `(,(mu:car frame)
                                     ,(mu:car frame)
                                     ,(mu:cdr frame))))
                   (mu:cdr env)))
              ()))
      (core:%exception-prop :cond exception)
      (core:%exception-prop :value exception)
      (core:%exception-prop :source exception)
      (core:%exception-prop :reason exception)
      (core:%exception-prop :env exception))))

(mu:intern core "with-exception"
  (:lambda (handler thunk)
     (mu:with-exception
      (:lambda (value condition source)
         (:if (mu:eq condition :except)
              (core:apply handler (mu:cons value ()))
              (core:apply handler `(,(core:%make-exception
                                      condition
                                      value
                                      source
                                      "mu:raise"
                                      (mu:%frame-stack))))))
      (:lambda () (core:apply thunk ())))))

;;;
;;; exception flavors
;;;
(mu:intern core "raise-exception"
   (:lambda (exception)
      (mu:raise exception :except)))
     
(mu:intern core "error"
   (:lambda (format-string arg-list)
      (core:%format mu:*error-output* format-string arg-list)
      (mu:raise
       (core:%make-exception :error arg-list :error "" ())
       :except)))

(mu:intern core "warn"
   (:lambda (format-string arg-list)
     (mu:write "WARNING: " () mu:*error-output*)
     (core:%format mu:*error-output* format-string arg-list)
     ()))

(mu:intern core "%warn"
   (:lambda (value message)
     (core:%format mu:*error-output* "WARNING: ~A [~A] ; ~S~%" `(,message ,(mu:type-of value) ,value))
     value))

(mu:intern core "%raise"
   (:lambda (value source reason)
     (mu:raise
      (core:%make-exception :error value source reason ())
      :except)))

(mu:intern core "%raise-env"
   (:lambda (value source reason)
     (mu:raise
      (core:%make-exception :error value source reason (mu:%frame-stack))
      :except)))
