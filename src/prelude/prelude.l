;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; namespace
;;;
(crux:intern (crux:find-ns "") "prelude" (crux:make-ns "prelude"))

;;;
;;; types
;;;
(crux:intern prelude "%init-ns"
    (:lambda ()
      ;;; function
      (prelude:define-type "%fn"
          '((:lambda  . :struct)
            (:arity   . :fixnum)
            (:fn      . :func)
            (:env     . :listp)))
      
      ;;; lambda
      (prelude:define-type "%lambda"
          '((:bound  . :listp)
            (:rest   . :symbolp)
            (:lexenv . :listp)))

      ;;; exception
      (prelude:define-type "%except"
          '((:cond   . :symbolp)
            (:value . :t)
            (:source . :symbolp)
            (:reason . :string)
            (:env    . :listp)))

      ;;; namespace
      (prelude:define-type "%ns"
          '((:name    . :string)
            (:intern  . :keyword)
            (:extern  . :keyword)
            (:ch-mac  . :keyword)
            (:sy-mac  . :keyword)
            (:uses    . :listp)))

      ;;; vector
      (prelude:define-type "%vector"
          '((:base    . :vector)
            (:disp    . :listp)
            (:bitvec  . :listp)))

      (crux:intern prelude "%char-macro-ns%" (prelude:%gen-anon-ns))
      (crux:intern prelude "%macro-ns%" (prelude:%gen-anon-ns))
      (crux:intern prelude "%symbol-macro-ns%" (prelude:%gen-anon-ns))))

;;;
;;; predicates
;;;
(crux:intern prelude "not" (:lambda (value) (prelude:null value)))
(crux:intern prelude "atom" (:lambda (value) (prelude:not (prelude:consp value))))

(crux:intern prelude "constantp"
   (:lambda (value)
      (:if (prelude:%orf (prelude:null value) (prelude:keywordp value))
           :t
           (:if (prelude:consp value)
                (crux:eq :quote (crux:car value))
                (prelude:not (prelude:symbolp value))))))

(crux:intern prelude "numberp"
  (:lambda (n)
     (:if (prelude:fixnump n)
          :t
          (prelude:floatp n))))

(crux:intern prelude "zerop"
   (:lambda (n)
     (:if (prelude:numberp n)
          (:if (crux:eq 0 n)
               :t
               (crux:eq 0.0 n))
          (prelude:raise n 'prelude:zerop "not a number"))))

(crux:intern prelude "minusp"
   (:lambda (n)
      (:if (prelude:numberp n)
           (:if (prelude:floatp n)
                (crux:fl-lt n 0.0)
                (crux:less-than n 0))
           (prelude:raise n 'prelude:minusp "not a number"))))

(crux:intern prelude "plusp"
   (:lambda (n)
      (:if (prelude:numberp n)
           (:if (prelude:floatp n)
                (crux:fl-lt 0.0 n)
                (crux:less-than 0 n))
           (prelude:raise n 'prelude:plusp "not a number"))))

;;;
;;; system classes
;;;
(crux:intern prelude "charp" (:lambda (t) (crux:eq :char (crux:type-of t))))
(crux:intern prelude "fixnump" (:lambda (t) (crux:eq :fixnum (crux:type-of t))))
(crux:intern prelude "floatp" (:lambda (t) (crux:eq :float (crux:type-of t))))
(crux:intern prelude "structp" (:lambda (t) (crux:eq :struct (crux:type-of t))))

;;;
;;; syntactic sugar
;;;
(crux:intern prelude "%orf"
   (:lambda (arg arg1)
     (:if arg arg arg1)))

(crux:intern prelude "%andf"
   (:lambda (arg arg1)
     (:if arg arg1 ())))

;;;
;;; apply
;;;
(crux:intern prelude "apply"
   (:lambda (function arg-list)
     (:if (prelude:functionp function)
          (:if (prelude:%prelude-function-p function)
               (prelude:%fn-apply function arg-list)
               (crux:apply function arg-list))
          (prelude:raise function 'prelude:apply "not a function designator"))))

;;;
;;; namespace components
;;;
(crux:intern prelude "%gen-anon-ns"
   (:lambda ()
      (crux:make-ns (crux:symbol-name (prelude:genkeyword #\%)))))
