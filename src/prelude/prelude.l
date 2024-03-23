;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; namespace
;;;
(core:make-ns :prelude)

;;;
;;; types
;;;
(core:intern :prelude "%init-ns"
   (:lambda ()
      ;;; function
      (prelude:define-type "funct"
          '((:lambda  . :struct)
            (:arity   . :fixnum)
            (:fn      . :func)
            (:env     . :listp)))

      ;;; lambda
      (prelude:define-type "lambda"
          '((:bound  . :listp)
            (:rest   . :symbolp)
            (:lexenv . :listp)))

      ;;; exception
      (prelude:define-type "except"
          '((:cond   . :symbolp)
            (:value . :t)
            (:source . :symbolp)
            (:reason . :string)
            (:env    . :listp)))

      ;;; namespace
      (prelude:define-type "namespc"
          '((:name    . :string)
            (:intern  . :keyword)
            (:extern  . :keyword)
            (:ch-mac  . :keyword)
            (:sy-mac  . :keyword)
            (:uses    . :listp)))

      ;;; vector
      (prelude:define-type "vector"
          '((:base    . :vector)
            (:disp    . :listp)
            (:bitvec  . :listp)))

      (core:intern :prelude "%char-macro-ns%" (prelude:%gen-anon-ns))
      (core:intern :prelude "%macro-ns%" (prelude:%gen-anon-ns))
      (core:intern :prelude "%symbol-macro-ns%" (prelude:%gen-anon-ns))))

;;;
;;; predicates
;;;
(core:intern :prelude "not" (:lambda (value) (prelude:null value)))
(core:intern :prelude "atom" (:lambda (value) (prelude:not (prelude:consp value))))

(core:intern :prelude "constantp"
   (:lambda (value)
      (:if (prelude:%orf (prelude:null value) (prelude:keywordp value))
           :t
           (:if (prelude:consp value)
                (core:eq :quote (core:car value))
                (prelude:not (prelude:symbolp value))))))

(core:intern :prelude "numberp"
  (:lambda (n)
     (:if (prelude:fixnump n)
          :t
          (prelude:floatp n))))

(core:intern :prelude "zerop"
   (:lambda (n)
     (:if (prelude:numberp n)
          (:if (core:eq 0 n)
               :t
               (core:eq 0.0 n))
          (prelude:raise n 'prelude:zerop "not a number"))))

(core:intern :prelude "minusp"
   (:lambda (n)
      (:if (prelude:numberp n)
           (:if (prelude:floatp n)
                (core:fl-lt n 0.0)
                (core:fx-lt n 0))
           (prelude:raise n 'prelude:minusp "not a number"))))

(core:intern :prelude "plusp"
   (:lambda (n)
      (:if (prelude:numberp n)
           (:if (prelude:floatp n)
                (core:fl-lt 0.0 n)
                (core:fx-lt 0 n))
           (prelude:raise n 'prelude:plusp "not a number"))))

;;;
;;; system classes
;;;
(core:intern :prelude "charp" (:lambda (t) (core:eq :char (core:type-of t))))
(core:intern :prelude "fixnump" (:lambda (t) (core:eq :fixnum (core:type-of t))))
(core:intern :prelude "floatp" (:lambda (t) (core:eq :float (core:type-of t))))
(core:intern :prelude "structp" (:lambda (t) (core:eq :struct (core:type-of t))))

;;;
;;; syntactic sugar
;;;
(core:intern :prelude "%orf"
   (:lambda (arg arg1)
     (:if arg arg arg1)))

(core:intern :prelude "%andf"
   (:lambda (arg arg1)
     (:if arg arg1 ())))

;;;
;;; apply
;;;
(core:intern :prelude "apply"
   (:lambda (function arg-list)
     (:if (prelude:functionp function)
          (:if (prelude:%prelude-function-p function)
               (prelude:%funct-apply function arg-list)
               (core:apply function arg-list))
          (prelude:raise function 'prelude:apply "not a function designator"))))

;;;
;;; namespace components
;;;
(core:intern :prelude "%gen-anon-ns"
   (:lambda ()
      (core:make-ns (prelude:genkeyword #\%))))
