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
(lib:intern prelude "exceptp"
    (:lambda (ex)
      (prelude:%andf
       (prelude:%prelude-type-p ex)
       (lib:eq '%except (prelude:type-of ex)))))

(lib:intern prelude "%make-except"
   (:lambda (cond value source reason env)
      (prelude:%make-type "%except"
        `(,(lib:cons :cond   cond)
          ,(lib:cons :value  value)
          ,(lib:cons :source source)
          ,(lib:cons :reason reason)
          ,(lib:cons :env    env)))))

(lib:intern prelude "%except-prop"
   (:lambda (prop except)
      (:if (prelude:findl (:lambda (lambda-prop) (lib:eq prop lambda-prop)) '(:cond :value :source :reason :env))
           (prelude:%type-ref prop except)
           (prelude:raise prop 'prelude:%except-prop "not an except property"))))

(lib:intern prelude "%exceptf"
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
                                   `(,(prelude:%format-mapped-symbol (lib:car frame))
                                     ,(lib:car frame)
                                     ,(lib:cdr frame))))
                   (lib:cdr env))))
              ()))
      (prelude:%except-prop :cond except)
      (prelude:%except-prop :value except)
      (prelude:%except-prop :source except)
      (prelude:%except-prop :reason except)
      (prelude:%except-prop :env except))))

(lib:intern prelude "with-exception"
  (:lambda (handler thunk)
     (lib:unwind-protect
      (:lambda (value condition source)
         (:if (lib:eq condition :except)
              (prelude:apply handler `(,value))
              (prelude:apply handler `(,(prelude:%make-except
                                      condition
                                      value
                                      source
                                      "lib:raise"
                                      (lib:frames))))))
      (:lambda () (prelude:apply thunk ())))))

;;;
;;; exception flavors
;;;
(lib:intern prelude "make-exception" prelude:%make-except)

(lib:intern prelude "error"
   (:lambda (value format-string arg-list)
      (prelude:format lib:error-output format-string arg-list)
      (lib:raise
       (prelude:%make-except :error value :error "" ())
       :except)))

(lib:intern prelude "raise-env"
   (:lambda (value source reason)
     (lib:raise
      (prelude:%make-except :error value source reason (lib:frames))
      :except)))

(lib:intern prelude "raise"
   (:lambda (value source reason)
     (lib:raise
      (prelude:%make-except :error value source reason ())
      :except)))

(lib:intern prelude "debug"
   (:lambda (value message)
     (lib:write ";;; debug: " () lib:error-output)          
     (lib:write value () lib:error-output)
     (lib:write " " () lib:error-output)
     (lib:write message () lib:error-output)
     (lib:write-char #\linefeed lib:error-output)
     value))

(lib:intern prelude "warn"
   (:lambda (value message)
     (prelude:format lib:error-output ";;; warning: ~S ~A~%" `(,value ,message))
    value))
