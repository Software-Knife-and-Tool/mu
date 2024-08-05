;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; exceptions
;;;

;;;
;;; [*exception descriptor*] #s(:%except cond value source reason env)
;;;
;;; cond:   condition type symbol
;;; value:  exception raised on value
;;; source: function designator, usually a symbol
;;; reason: extended condition, usually a string
;;; env:    dynamic environment, list of frames
;;;
(mu:intern prelude "exceptionp"
    (:lambda (ex)
      (prelude:%andf
       (prelude:%prelude-type-p ex)
       (mu:eq '%except (prelude:type-of ex)))))

(mu:intern prelude "%exception-prop"
   (:lambda (prop exception)
      (:if (prelude:findl (:lambda (lambda-prop) (mu:eq prop lambda-prop)) '(:cond :value :source :reason :env))
           (prelude:%type-ref prop exception)
           (prelude:raise prop 'prelude:%exception-prop "not an exception property"))))

(mu:intern prelude "%exceptionf"
   (:lambda (stream format-string printenv exception)
     ((:lambda (cond value source reason env)
         (:if (prelude:null format-string)
              (prelude:format stream "~S ~S ~S ~S~%" `(,cond ,value ,source ,reason))
              (prelude:format stream format-string `(,cond ,value ,source ,reason)))
         (:if (prelude:%andf printenv env)
              ((:lambda ()
                  (prelude:format stream "env:~%" ())
                  (prelude:%mapc
                   (:lambda (frame)
                      (prelude:format stream "  ~A~T(~A ~A)~%"
                                   `(,(prelude:%format-mapped-symbol (mu:car frame))
                                     ,(mu:car frame)
                                     ,(mu:cdr frame))))
                   (mu:cdr env))))
              ()))
      (prelude:%exception-prop :cond exception)
      (prelude:%exception-prop :value exception)
      (prelude:%exception-prop :source exception)
      (prelude:%exception-prop :reason exception)
      (prelude:%exception-prop :env exception))))

(mu:intern prelude "with-exception"
  (:lambda (handler thunk)
     (mu:with-exception
      (:lambda (value condition source)
         (:if (mu:eq condition :except)
              (prelude:apply handler `(,value))
              (prelude:apply handler `(,(prelude:make-exception
                                           condition
                                           value
                                           source
                                           "mu:raise"
                                           (mu:dynamic-env))))))
      (:lambda () (prelude:apply thunk ())))))

(mu:intern prelude "make-exception"
   (:lambda (cond value source reason env)
      (prelude:%make-type "%except"
        `(,(mu:cons :cond   cond)
          ,(mu:cons :value  value)
          ,(mu:cons :source source)
          ,(mu:cons :reason reason)
          ,(mu:cons :env    env)))))

;;;
;;; exception flavors
;;;
(mu:intern prelude "error"
   (:lambda (value format-string arg-list)
      (prelude:format mu:*error-output* format-string arg-list)
      (mu:raise
       (prelude:make-exception :error value :error "" ())
       :except)))

(mu:intern prelude "raise-env"
   (:lambda (value source reason)
     (mu:raise
      (prelude:make-exception :error value source reason (mu:dynamic-env))
      :except)))

(mu:intern prelude "raise"
   (:lambda (value source reason)
     (mu:raise
      (prelude:make-exception :error value source reason ())
      :except)))

(mu:intern prelude "debug"
   (:lambda (value message)
     (mu:write ";;; debug: " () mu:*error-output*)          
     (mu:write value () mu:*error-output*)
     (mu:write " " () mu:*error-output*)
     (mu:write message () mu:*error-output*)
     (mu:write-char #\linefeed mu:*error-output*)
     value))

(mu:intern prelude "eprint"
   (:lambda (value message)          
     (prelude:format mu:*error-output* ";;; ~A:~T ~A~%" `(,message ,value))
    value))

(mu:intern prelude "warn"
   (:lambda (value message)
     (prelude:format mu:*error-output* ";;; warning: ~S ; ~A, type ~A~%" `(,value ,message ,(mu:type-of value)))
    value))
