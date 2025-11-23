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
;;; predicate
;;;
(mu:intern core "exceptionp"
   (:lambda (ex)
     (core:%and (core:%typep ex) (mu:eq "%exception" (mu:symbol-name (core:type-of ex))))))

;;;
;;; functions
;;;
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
     
(mu:intern core "%warn"
   (:lambda (value message)
     (core:write "warning: " () mu:*error-output*)
     (core:write message () mu:*error-output*)
     (core:write " [" () mu:*error-output*)
     (core:write (mu:type-of value) () mu:*error-output*)
     (core:write "] " () mu:*error-output*)
     (core:write value () mu:*error-output*)
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
