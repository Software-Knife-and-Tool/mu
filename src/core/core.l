;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core namespace
;;;
(mu:intern (mu:find-namespace "") "core" (mu:make-namespace "core"))

(mu:intern core "*version*" "0.0.2")

(mu:intern core "%char-macro-ns%" (mu:make-namespace "%char-macros"))
(mu:intern core "%macro-ns%" (mu:make-namespace "%macros"))
(mu:intern core "%symbol-macro-ns%" (mu:make-namespace "%symbol-macros"))
(mu:intern core "%types-ns%" (mu:make-namespace "%types%"))

;;;
;;; predicates
;;;
(mu:intern core "not" (:lambda (value) (core:null value)))
(mu:intern core "atom" (:lambda (value) (core:not (core:consp value))))

(mu:intern core "constantp"
   (:lambda (value)
      (:if (core:%orf (core:null value) (core:keywordp value))
           :t
           (:if (core:consp value)
                (mu:eq :quote (mu:car value))
                (core:not (core:symbolp value))))))

(mu:intern core "numberp"
  (:lambda (n)
     (:if (core:fixnump n)
          :t
          (core:floatp n))))

(mu:intern core "zerop"
   (:lambda (n)
     (:if (core:numberp n)
          (:if (mu:eq 0 n)
               :t
               (mu:eq 0.0 n))
          (core:raise n 'core:zerop "not a number"))))

(mu:intern core "minusp"
   (:lambda (n)
      (:if (core:numberp n)
           (:if (core:floatp n)
                (mu:fless-than n 0.0)
                (mu:less-than n 0))
           (core:raise n 'core:minusp "not a number"))))

(mu:intern core "plusp"
   (:lambda (n)
      (:if (core:numberp n)
           (:if (core:floatp n)
                (mu:fless-than 0.0 n)
                (mu:less-than 0 n))
           (core:raise n 'core:plusp "not a number"))))

(mu:intern core "packagep" (:lambda (t) (mu:eq (core:type-of t) 'package)))

;;;
;;; mu system classes
;;;
(mu:intern core "charp" (:lambda (t) (mu:eq :char (mu:type-of t))))
(mu:intern core "consp" (:lambda (t) (mu:eq :cons (mu:type-of t))))
(mu:intern core "fixnump" (:lambda (t) (mu:eq :fixnum (mu:type-of t))))
(mu:intern core "floatp" (:lambda (t) (mu:eq :float (mu:type-of t))))
(mu:intern core "listp" (:lambda (t) (:if (core:consp t) :t (core:null t))))
(mu:intern core "namespacep" (:lambda (t) (mu:eq :ns (mu:type-of t))))
(mu:intern core "null" (:lambda (t) (mu:eq () t)))
(mu:intern core "streamp" (:lambda (t) (mu:eq :stream (mu:type-of t))))
(mu:intern core "structp" (:lambda (t) (mu:eq :struct (mu:type-of t))))

;;;
;;; syntactic sugar
;;;
(mu:intern core "%orf"
   (:lambda (arg arg1)
     (:if arg arg arg1)))

(mu:intern core "%andf"
   (:lambda (arg arg1)
     (:if arg arg1 ())))
