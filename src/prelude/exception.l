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
(core:intern prelude "exceptp"
    (:lambda (ex)
      (prelude:%andf
       (prelude:%prelude-type-p ex)
       (core:eq '%except (prelude:type-of ex)))))

(core:intern prelude "%make-except"
   (:lambda (cond value source reason env)
      (prelude:%make-type "%except"
        `(,(core:cons :cond   cond)
          ,(core:cons :value  value)
          ,(core:cons :source source)
          ,(core:cons :reason reason)
          ,(core:cons :env    env)))))

(core:intern prelude "%except-prop"
   (:lambda (prop except)
      (:if (prelude:findl (:lambda (lambda-prop) (core:eq prop lambda-prop)) '(:cond :value :source :reason :env))
           (prelude:%type-ref prop except)
           (prelude:raise prop 'prelude:%except-prop "not an except property"))))

(core:intern prelude "%exceptf"
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
                                   `(,(prelude:%format-mapped-symbol (core:car frame))
                                     ,(core:car frame)
                                     ,(core:cdr frame))))
                   (core:cdr env))))
              ()))
      (prelude:%except-prop :cond except)
      (prelude:%except-prop :value except)
      (prelude:%except-prop :source except)
      (prelude:%except-prop :reason except)
      (prelude:%except-prop :env except))))

(core:intern prelude "with-exception"
  (:lambda (handler thunk)
     (core:unwind-protect
      (:lambda (value condition source)
         (:if (core:eq condition :except)
              (prelude:apply handler `(,value))
              (prelude:apply handler `(,(prelude:%make-except
                                      condition
                                      value
                                      source
                                      "core:raise"
                                      (core:frames))))))
      (:lambda () (prelude:apply thunk ())))))

;;;
;;; exception flavors
;;;
(core:intern prelude "make-exception" prelude:%make-except)

(core:intern prelude "error"
   (:lambda (value format-string arg-list)
      (prelude:format core:error-output format-string arg-list)
      (core:raise
       (prelude:%make-except :error value :error "" ())
       :except)))

(core:intern prelude "raise-env"
   (:lambda (value source reason)
     (core:raise
      (prelude:%make-except :error value source reason (core:frames))
      :except)))

(core:intern prelude "raise"
   (:lambda (value source reason)
     (core:raise
      (prelude:%make-except :error value source reason ())
      :except)))

(core:intern prelude "debug"
   (:lambda (value message)
     (core:write ";;; debug: " () core:error-output)          
     (core:write value () core:error-output)
     (core:write " " () core:error-output)
     (core:write message () core:error-output)
     (core:write-char #\linefeed core:error-output)
     value))

(core:intern prelude "warn"
   (:lambda (value message)
     (prelude:format core:error-output ";;; warning: ~S ~A~%" `(,value ,message))
    value))
