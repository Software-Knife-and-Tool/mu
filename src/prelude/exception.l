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
(crux:intern prelude "exceptp"
    (:lambda (ex)
      (prelude:%andf
       (prelude:%prelude-type-p ex)
       (crux:eq '%except (prelude:type-of ex)))))

(crux:intern prelude "%make-except"
   (:lambda (cond value source reason env)
      (prelude:%make-type "%except"
        `(,(crux:cons :cond   cond)
          ,(crux:cons :value  value)
          ,(crux:cons :source source)
          ,(crux:cons :reason reason)
          ,(crux:cons :env    env)))))

(crux:intern prelude "%except-prop"
   (:lambda (prop except)
      (:if (prelude:findl (:lambda (lambda-prop) (crux:eq prop lambda-prop)) '(:cond :value :source :reason :env))
           (prelude:%type-ref prop except)
           (prelude:raise prop 'prelude:%except-prop "not an except property"))))

(crux:intern prelude "%exceptf"
   (:lambda (stream format-string printenv except)
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
                                   `(,(prelude:%format-mapped-symbol (crux:car frame))
                                     ,(crux:car frame)
                                     ,(crux:cdr frame))))
                   (crux:cdr env))))
              ()))
      (prelude:%except-prop :cond except)
      (prelude:%except-prop :value except)
      (prelude:%except-prop :source except)
      (prelude:%except-prop :reason except)
      (prelude:%except-prop :env except))))

(crux:intern prelude "with-exception"
  (:lambda (handler thunk)
     (crux:unwind-protect
      (:lambda (value condition source)
         (:if (crux:eq condition :except)
              (prelude:apply handler `(,value))
              (prelude:apply handler `(,(prelude:%make-except
                                      condition
                                      value
                                      source
                                      "crux:raise"
                                      (crux:frames))))))
      (:lambda () (prelude:apply thunk ())))))

;;;
;;; exception flavors
;;;
(crux:intern prelude "make-exception" prelude:%make-except)

(crux:intern prelude "error"
   (:lambda (value format-string arg-list)
      (prelude:format crux:*error-output* format-string arg-list)
      (crux:raise
       (prelude:%make-except :error value :error "" ())
       :except)))

(crux:intern prelude "raise-env"
   (:lambda (value source reason)
     (crux:raise
      (prelude:%make-except :error value source reason (crux:frames))
      :except)))

(crux:intern prelude "raise"
   (:lambda (value source reason)
     (crux:raise
      (prelude:%make-except :error value source reason ())
      :except)))

(crux:intern prelude "debug"
   (:lambda (value message)
     (crux:write ";;; debug: " () crux:*error-output*)          
     (crux:write value () crux:*error-output*)
     (crux:write " " () crux:*error-output*)
     (crux:write message () crux:*error-output*)
     (crux:write-char #\linefeed crux:*error-output*)
     value))

(crux:intern prelude "warn"
   (:lambda (value message)
     (prelude:format crux:*error-output* ";;; warning: ~S ~A~%" `(,value ,message))
    value))
