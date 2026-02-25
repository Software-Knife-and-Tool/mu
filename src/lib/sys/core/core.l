;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core namespaces
;;;
(mu:intern core "%reader/" (mu:make-namespace "core/reader"))
(mu:intern core "%char-macros/" (mu:make-namespace "core/char-macros"))
(mu:intern core "%closures/" (mu:make-namespace "core/closures"))
(mu:intern core "%macros/" (mu:make-namespace "core/macros"))
(mu:intern core "%symbol-macros/" (mu:make-namespace "core/symbol-macros"))
(mu:intern core "%types/" (mu:make-namespace "core/core-types"))

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
           (mu:raise-from n 'core:minusp :type))))

(mu:intern core "stringp"
   (:lambda (string)
      (:if (mu:eq (mu:type-of string) :vector)
           (mu:eq :char (mu:vector-type string))
           (:if (core:%typep string)
                (:if (mu:eq "vector" (mu:svref (mu:struct-vec string) 0))
                     (mu:eq :char (mu:vector-type (core:%vector-prop :base string)))
                     ())
                ()))))

;;;
;;; mu system classes
;;;
(mu:intern core "charp" (:lambda (t) (mu:eq :char (mu:type-of t))))
(mu:intern core "consp" (:lambda (t) (mu:eq :cons (mu:type-of t))))
(mu:intern core "fixnump" (:lambda (t) (mu:eq :fixnum (mu:type-of t))))
(mu:intern core "floatp" (:lambda (t) (mu:eq :float (mu:type-of t))))
(mu:intern core "keywordp" (:lambda (t) (core:%or (mu:eq :keyword (mu:type-of t)) (mu:eq (core:type-of t) 'keyword))))
(mu:intern core "namespacep" (:lambda (t) (mu:eq :ns (mu:type-of t))))
(mu:intern core "streamp" (:lambda (t) (mu:eq :stream (mu:type-of t))))
(mu:intern core "structp" (:lambda (t) (mu:eq :struct (mu:type-of t))))

;;;
;;; eval
;;;
(mu:intern core "eval"
   (:lambda (form)
     (mu:eval (core:compile form))))

;;;
;;; apply 
;;;
(mu:intern core "apply"
   (:lambda (function arg-list)
     (:if (core:%genericp function)
          (core:%apply-generic function arg-list)
          (:if (core:functionp function)
               (:if (core:%closurep function)
                    (core:%apply
                     function
                     (core:%closure-prop :mu function)
                     (mu:eval (core:%compile-lambda-arg-list function arg-list ())))
                    (mu:apply function arg-list))
               (mu:raise-from function 'core:apply :type)))))

(mu:intern core "%apply"
   (:lambda (closure frame arg-list-form)
     ((:lambda (env)
        (mu:apply frame arg-list-form))
      (core:%closure-prop :env closure))))

;;;
;;; syntactic sugar
;;;
(mu:intern core "%quote"
   (:lambda (form)
     (mu:cons :quote form)))
               
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
