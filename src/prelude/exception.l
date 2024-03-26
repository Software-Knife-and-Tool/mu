;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; exceptions
;;;
(libcore:untern :prelude "%debug-symbols%")

;;;
;;; [*exception descriptor*] #s(:except cond value source reason env)
;;;
;;; cond:   condition type symbol
;;; value:  exception raised on value
;;; source: function designator, usually a symbol
;;; reason: extended condition, usually a string
;;; env:    dynamic environment, list of frames
;;;
(libcore:intern :prelude "exceptp"
   (:lambda (ex)
      (libcore:eq (prelude:type-of ex) 'except)))

(libcore:intern :prelude "%make-except"
   (:lambda (cond value source reason env)
      (prelude:%make-type "except"
        `(,(libcore:cons :cond   cond)
          ,(libcore:cons :value  value)
          ,(libcore:cons :source source)
          ,(libcore:cons :reason reason)
          ,(libcore:cons :env    env)))))

(libcore:intern :prelude "%except-prop"
   (:lambda (prop except)
      (:if (prelude:findl (:lambda (lambda-prop) (libcore:eq prop lambda-prop)) '(:cond :value :source :reason :env))
           (prelude:%type-ref prop except)
           (prelude:raise prop 'prelude:%except-prop "not an except property"))))

(libcore:intern :prelude "%exceptf"
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
                                   `(,(prelude:%format-mapped-symbol (libcore:car frame))
                                     ,(libcore:car frame)
                                     ,(libcore:cdr frame))))
                   (libcore:cdr env))))
              ()))
      (prelude:%except-prop :cond except)
      (prelude:%except-prop :value except)
      (prelude:%except-prop :source except)
      (prelude:%except-prop :reason except)
      (prelude:%except-prop :env except))))

(libcore:intern :prelude "with-exception"
  (:lambda (handler thunk)
     (libcore:with-ex
      (:lambda (value condition source)
         (:if (libcore:eq condition :except)
              (prelude:apply handler `(,value))
              (prelude:apply handler `(,(prelude:%make-except
                                      condition
                                      value
                                      source
                                      "libcore:raise"
                                      (libcore:frames))))))
      (:lambda () (prelude:apply thunk ())))))

;;;
;;; exception flavors
;;;
(libcore:intern :prelude "make-exception" prelude:%make-except)

(libcore:intern :prelude "error"
   (:lambda (value format-string arg-list)
      (prelude:format libcore:err-out format-string arg-list)
      (libcore:raise
       (prelude:%make-except :error value :error "" ())
       :except)))

(libcore:intern :prelude "raise"
   (:lambda (value source reason verbose)
      (:if verbose          
           (libcore:raise
            (prelude:%make-except :error value source reason (libcore:frames))
            :except)
           (libcore:raise
            (prelude:%make-except :error value source reason ())
            :except))))

(libcore:intern :prelude "debug"
   (:lambda (value message)
     (libcore:write ";;; debug: " () libcore:err-out)          
     (libcore:write value () libcore:err-out)
     (libcore:write " " () libcore:err-out)
     (libcore:write message () libcore:err-out)
     (libcore:wr-char #\linefeed libcore:err-out)
     value))

(libcore:intern :prelude "warn"
   (:lambda (value message)
     (prelude:format libcore:err-out ";;; warning: ~S ~A~%" `(,value ,message))
    value))
