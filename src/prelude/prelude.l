;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; namespace
;;;
(lib:make-ns :prelude)

;;;
;;; types
;;;
(lib:intern :prelude "%init-ns"
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

      (lib:intern :prelude "%char-macro-ns%" (prelude:%gen-anon-ns))
      (lib:intern :prelude "%macro-ns%" (prelude:%gen-anon-ns))
      (lib:intern :prelude "%symbol-macro-ns%" (prelude:%gen-anon-ns))))

;;;
;;; predicates
;;;
(lib:intern :prelude "not" (:lambda (value) (prelude:null value)))
(lib:intern :prelude "atom" (:lambda (value) (prelude:not (prelude:consp value))))

(lib:intern :prelude "constantp"
   (:lambda (value)
      (:if (prelude:%orf (prelude:null value) (prelude:keywordp value))
           :t
           (:if (prelude:consp value)
                (lib:eq :quote (lib:car value))
                (prelude:not (prelude:symbolp value))))))

(lib:intern :prelude "numberp"
  (:lambda (n)
     (:if (prelude:fixnump n)
          :t
          (prelude:floatp n))))

(lib:intern :prelude "zerop"
   (:lambda (n)
     (:if (prelude:numberp n)
          (:if (lib:eq 0 n)
               :t
               (lib:eq 0.0 n))
          (prelude:raise n 'prelude:zerop "not a number"))))

(lib:intern :prelude "minusp"
   (:lambda (n)
      (:if (prelude:numberp n)
           (:if (prelude:floatp n)
                (lib:fl-lt n 0.0)
                (lib:fx-lt n 0))
           (prelude:raise n 'prelude:minusp "not a number"))))

(lib:intern :prelude "plusp"
   (:lambda (n)
      (:if (prelude:numberp n)
           (:if (prelude:floatp n)
                (lib:fl-lt 0.0 n)
                (lib:fx-lt 0 n))
           (prelude:raise n 'prelude:plusp "not a number"))))

;;;
;;; system classes
;;;
(lib:intern :prelude "charp" (:lambda (t) (lib:eq :char (lib:type-of t))))
(lib:intern :prelude "fixnump" (:lambda (t) (lib:eq :fixnum (lib:type-of t))))
(lib:intern :prelude "floatp" (:lambda (t) (lib:eq :float (lib:type-of t))))
(lib:intern :prelude "structp" (:lambda (t) (lib:eq :struct (lib:type-of t))))

;;;
;;; syntactic sugar
;;;
(lib:intern :prelude "%orf"
   (:lambda (arg arg1)
     (:if arg arg arg1)))

(lib:intern :prelude "%andf"
   (:lambda (arg arg1)
     (:if arg arg1 ())))

;;;
;;; apply
;;;
(lib:intern :prelude "apply"
   (:lambda (function arg-list)
     (:if (prelude:functionp function)
          (:if (prelude:%prelude-function-p function)
               (prelude:%funct-apply function arg-list)
               (lib:apply function arg-list))
          (prelude:raise function 'prelude:apply "not a function designator"))))

;;;
;;; namespace components
;;;
(lib:intern :prelude "%gen-anon-ns"
   (:lambda ()
      (lib:make-ns (prelude:genkeyword #\%))))
