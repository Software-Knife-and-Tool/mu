;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; env deftype
;;;

;;;
;;;  (:frame   . :fixnum)     frame function id
;;;  (:symbols . :listp)      lexical symbols
;;;  (:name    . :symbol)     closure symbol
;;;
(core:%deftype "%env"
    '((:frame   . :fixnum)
      (:symbols . :listp)
      (:name    . :symbol)))

(mu:intern core "%env-prop"
   (:lambda (prop env)
     (mu:cdr (core:%type-prop prop env))))

(mu:intern core "%make-env"
   (:lambda (frame symbols name)
     (core:%make-type "%closure"
       `(,(mu:cons :frame     frame)
          ,(mu:cons :symbols  symbols)
          ,(mu:cons :name     name)))))

(mu:intern core "%envp"
   (:lambda (function)
     (:if (core:%typep function)
          (mu:eq '%env (core:type-of function))
          ())))
