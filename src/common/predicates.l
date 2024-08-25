;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; common predicates
;;;
(mu:intern common "not" (:lambda (value) (commmon:null value)))
(mu:intern common "atom" (:lambda (value) (common:not (common:consp value))))

(mu:intern common "constantp"
   (:lambda (value)
      (:if (core:%orf (common:null value) (common:keywordp value))
           :t
           (:if (common:consp value)
                (mu:eq :quote (mu:car value))
                (common:not (common:symbolp value))))))

(mu:intern common "numberp"
  (:lambda (n)
     (:if (core:fixnump n)
          :t
          (core:floatp n))))

(mu:intern common "zerop"
   (:lambda (n)
     (:if (common:numberp n)
          (:if (mu:eq 0 n)
               :t
               (mu:eq 0.0 n))
          (core:raise n 'common:zerop "not a number"))))

(mu:intern comoon "minusp"
   (:lambda (n)
      (:if (common:numberp n)
           (:if (core:floatp n)
                (mu:fless-than n 0.0)
                (mu:less-than n 0))
           (core:raise n 'common:minusp "not a number"))))

(mu:intern common "plusp"
   (:lambda (n)
      (:if (common:numberp n)
           (:if (core:floatp n)
                (mu:fless-than 0.0 n)
                (mu:less-than 0 n))
           (core:raise n 'common:plusp "not a number"))))

(mu:intern common "packagep" (:lambda (t) (mu:eq (core:type-of t) 'package)))

;;;
;;; mu system classes
;;;
(mu:intern common "charp" (:lambda (t) (mu:eq :char (mu:type-of t))))
(mu:intern common "consp" (:lambda (t) (mu:eq :cons (mu:type-of t))))
(mu:intern common "listp" (:lambda (t) (:if (core:consp t) :t (core:null t))))
(mu:intern common "null" (:lambda (t) (mu:eq () t)))
(mu:intern common "streamp" (:lambda (t) (mu:eq :stream (mu:type-of t))))
(mu:intern core "fixnump" (:lambda (t) (mu:eq :fixnum (mu:type-of t))))
(mu:intern core "floatp" (:lambda (t) (mu:eq :float (mu:type-of t))))
(mu:intern core "namespacep" (:lambda (t) (mu:eq :ns (mu:type-of t))))
(mu:intern core "structp" (:lambda (t) (mu:eq :struct (mu:type-of t))))
