;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; common predicates
;;;
(mu:intern common "not" core:null)
(mu:intern common "atom" (:lambda (value) (core:not (core:%consp value))))

(mu:intern common "constantp"
   (:lambda (value)
      (:if (core:%orf (common:null value) (common:keywordp value))
           :t
           (:if (common:consp value)
                (mu:eq :quote (mu:car value))
                (common:not (common:symbolp value))))))

(mu:intern common "numberp"
  (:lambda (n)
     (:if (core:%fixnump n)
          :t
          (core:%floatp n))))

(mu:intern common "zerop"
   (:lambda (n)
     (:if (common:numberp n)
          (:if (mu:eq 0 n)
               :t
               (mu:eq 0.0 n))
          (core:raise n 'common:zerop "not a number"))))

(mu:intern common "minusp"
   (:lambda (n)
      (:if (common:numberp n)
           (:if (core:%floatp n)
                (mu:fless-than n 0.0)
                (mu:less-than n 0))
           (core:raise n 'common:minusp "not a number"))))

(mu:intern common "plusp"
   (:lambda (n)
      (:if (common:numberp n)
           (:if (core:%floatp n)
                (mu:fless-than 0.0 n)
                (mu:less-than 0 n))
           (core:raise n 'common:plusp "not a number"))))

(mu:intern common "packagep" (:lambda (t) (mu:eq (core:type-of t) 'package)))

#|
(mu:intern common "eql" (x y)
   (core:%cond
     `(((mu:eq ,x ,y) :t)
       ((core:null (mu:eq (core:type-of ,x) (core:type-of ,y))) ())
       ((core:stringp ,x) (core:string= ,x ,y))

       ((core:%andf (core:%andf (core:vectorp ,x) (core:vectorp ,y))
                    (core:%andf (core:%andf (mu:eq (mu:vector-type ,x) (mu:vector-type ,y)))
                                (core:%andf (mu:eq (mu:vector-length ,x) (mu:vector-length ,y)))))
       (core:apply common:eql (vector-to-list x) (vector-to-list y)))
       ((core:%andf (core:%andf (core:consp ,x) (core:consp ,y))
                    (core:%andf (mu:eq (core:length ,x) (core:length ,y))))
        (core:%andf (core:apply common:eql (car ,x) (car ,y))
                    (core:apply common:eql (mu:cdr x) (mu:cdr ,y))))
       (:t ()))))

(defun eql (x y)
   (cond
     ((eq x y) t)
     ((null (eq (type-of x) (type-of y))) nil)
     ((stringp x) (string= x y))
     ((and (vectorp x) (vectorp y))
        (and (eq (vector-type x) (vector-type y))
             (eq (vector-length x) (vector-length y))
             (core::funcall eql (vector-to-list x) (vector-to-list y))))
     ((consp x)
        (and (eq (length x) (length y))
             (core::funcall eql (car x) (car y))
             (core::funcall eql (cdr x) (cdr y))))
     (t ())))
|#

;;;
;;; mu system classes
;;;
(mu:intern common "charp" (:lambda (t) (mu:eq :char (mu:type-of t))))
(mu:intern common "consp" (:lambda (t) (mu:eq :cons (mu:type-of t))))
(mu:intern common "listp" (:lambda (t) (:if (core:consp t) :t (core:null t))))
(mu:intern common "null" (:lambda (t) (mu:eq () t)))
(mu:intern common "streamp" (:lambda (t) (mu:eq :stream (mu:type-of t))))
(mu:intern common "fixnump" (:lambda (t) (mu:eq :fixnum (mu:type-of t))))
(mu:intern common "floatp" (:lambda (t) (mu:eq :float (mu:type-of t))))
(mu:intern common "namespacep" (:lambda (t) (mu:eq :ns (mu:type-of t))))
(mu:intern common "structp" (:lambda (t) (mu:eq :struct (mu:type-of t))))
