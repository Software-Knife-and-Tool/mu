;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core namespaces
;;;
(mu:intern core "%char-macro-ns%" (mu:make-namespace "%char-macros%"))
(mu:intern core "%macro-ns%" (mu:make-namespace "%macros%"))
(mu:intern core "%modules-ns%" (mu:make-namespace "%modules%"))
(mu:intern core "%symbol-macro-ns%" (mu:make-namespace "%symbol-macros%"))
(mu:intern core "%types-ns%" (mu:make-namespace "%core-types%"))
(mu:intern core "%closure-ns%" (mu:make-namespace "%closure-ns%"))

;;;
;;; predicates
;;;
(mu:intern core "null" (:lambda (t) (mu:eq () t)))
(mu:intern core "listp" (:lambda (t) (:if (core:consp t) :t (mu:eq () t))))

(mu:intern core "numberp"
  (:lambda (n)
     (:if (core:fixnump n)
          :t
          (core:floatp n))))

(mu:intern core "minusp"
   (:lambda (n)
      (:if (core:numberp n)
           (:if (core:floatp n)
                (mu:fless-than n 0.0)
                (mu:less-than n 0))
           (core:raise n 'core:minusp "not a number"))))

(mu:intern core "eql"
   (:lambda (x y)
     (:if (mu:eq x y)
          :t
          (:if (mu:eq (mu:type-of x) (mu:type-of y))
               (:if (core:%and (core:stringp x) (core:stringp y))
                    (core:%string= x y)
                    (:if (core:%and (core:vectorp x) (core:vectorp y))
                         (core:%and
                          (core:%and (core:vector-type x) (core:vector-type y))
                          (core:eql (core:%vector-to-list x) (core:%vector-to-list y)))
                         (:if (core:consp x)
                              (core:%and
                               (core:eql (mu:car x) (mu:car y))
                               (core:eql (mu:cdr x) (mu:cdr y)))
                              ())))
               ()))))
;;;
;;; mu system classes
;;;
(mu:intern core "charp" (:lambda (t) (mu:eq :char (mu:type-of t))))
(mu:intern core "consp" (:lambda (t) (mu:eq :cons (mu:type-of t))))
(mu:intern core "fixnump" (:lambda (t) (mu:eq :fixnum (mu:type-of t))))
(mu:intern core "floatp" (:lambda (t) (mu:eq :float (mu:type-of t))))
(mu:intern core "namespacep" (:lambda (t) (mu:eq :ns (mu:type-of t))))
(mu:intern core "streamp" (:lambda (t) (mu:eq :stream (mu:type-of t))))
(mu:intern core "structp" (:lambda (t) (mu:eq :struct (mu:type-of t))))
(mu:intern core "keywordp" (:lambda (t) (mu:eq (core:type-of t) 'keyword)))

;;;
;;; eval
;;;
(mu:intern core "eval"
   (:lambda (form)
     (mu:eval (core:compile form))))

;;;
;;; syntactic sugar
;;;
(mu:intern core "%quote"
   (:lambda (form)
     (mu:cons :quote form)))
               
(mu:intern core "%not" core:null)

(mu:intern core "%or"
   (:lambda (arg arg1)
     (:if arg arg arg1)))

(mu:intern core "%and"
   (:lambda (arg arg1)
     (:if arg arg1 ())))

(mu:intern core "%prog2"
   (:lambda (arg1 arg2)
     arg2))

(mu:intern core "%list"
   (:lambda (form)
     (mu:cons form ())))

(mu:intern core "%list2"
   (:lambda (arg1 arg2)
     (mu:cons arg1 (mu:cons arg2 ()))))
