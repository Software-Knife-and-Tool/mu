;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; exceptions
;;;

;;;
;;; functions
;;;
(mu:intern core "%exceptionf"
   (:lambda (stream format-string printenv exception)
     ((:lambda (cond value source reason env)
         (:if (core:null format-string)
              (core:format stream "~S ~S ~S ~S~%" `(,cond ,value ,source ,reason))
              (core:format stream format-string `(,cond ,value ,source ,reason)))
         (:if (core:%andf printenv env)
              ((:lambda ()
                  (core:format stream "env:~%" ())
                  (core:%%mapc
                   (:lambda (frame)
                      (core:format stream "  ~A~T(~A ~A)~%"
                                   `(,(core:%format-mapped-symbol (mu:car frame))
                                     ,(mu:car frame)
                                     ,(mu:cdr frame))))
                   (mu:cdr env))))
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
              (core:apply handler `(,value))
              (core:apply handler `(,(core:make-exception
                                           condition
                                           value
                                           source
                                           "mu:raise"
                                           (mu:%frame-stack))))))
      (:lambda () (core:apply thunk ())))))

(mu:intern core "make-exception"
   (:lambda (cond value source reason env)
      (core:%make-type "%except"
        `(,(mu:cons :cond   cond)
          ,(mu:cons :value  value)
          ,(mu:cons :source source)
          ,(mu:cons :reason reason)
          ,(mu:cons :env    env)))))

;;;
;;; exception flavors
;;;
(mu:intern core "error"
   (:lambda (value format-string arg-list)
      (core:format mu:*error-output* format-string arg-list)
      (mu:raise
       (core:make-exception :error value :error "" ())
       :except)))

(mu:intern core "raise"
   (:lambda (value source reason)
     (mu:raise
      (core:make-exception :error value source reason ())
      :except)))

(mu:intern core "raise-env"
   (:lambda (value source reason)
     (mu:raise
      (core:make-exception :error value source reason (mu:%frame-stack))
      :except)))

(mu:intern core "debug"
   (:lambda (value message)
     (mu:write ";;; debug: " () mu:*error-output*)          
     (mu:write value () mu:*error-output*)
     (mu:write " " () mu:*error-output*)
     (mu:write message () mu:*error-output*)
     (mu:write-char #\linefeed mu:*error-output*)
     value))

(mu:intern core "eprint"
   (:lambda (value message)          
     (core:format mu:*error-output* ";;; ~A:~T ~A~%" `(,message ,value))
     value))

(mu:intern core "warn"
   (:lambda (value message)
     (core:format mu:*error-output* ";;; warning: ~S ; ~A, type ~A~%" `(,value ,message ,(mu:type-of value)))
    value))

;;;
;;; [*exception descriptor*] #s(:%except cond value source reason env)
;;;
;;; cond:   condition type symbol
;;; value:  exception raised on value
;;; source: function designator, usually a symbol
;;; reason: extended condition, usually a string
;;; env:    dynamic environment, list of frames
;;;
(mu:intern core "exceptionp"
    (:lambda (ex)
      (core:%andf
       (core:%core-type-p ex)
       (mu:eq '%except (core:type-of ex)))))

(core:define-type "%except"
    '((:cond   . :symbolp)
      (:value  . :t)
      (:source . :symbolp)
      (:reason . :string)
      (:env    . :listp)))

(mu:intern core "%exception-prop"
   (:lambda (prop exception)
      (:if (core:findl (:lambda (lambda-prop) (mu:eq prop lambda-prop)) '(:cond :value :source :reason :env))
           (core:%type-ref prop exception)
           (core:raise prop 'core:%exception-prop "not an exception property"))))
