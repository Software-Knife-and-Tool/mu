;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; namespace
;;;
(libcore:make-ns :prelude)

;;;
;;; types
;;;
(libcore:intern :prelude "%init-ns"
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

      (libcore:intern :prelude "%char-macro-ns%" (prelude:%gen-anon-ns))
      (libcore:intern :prelude "%macro-ns%" (prelude:%gen-anon-ns))
      (libcore:intern :prelude "%symbol-macro-ns%" (prelude:%gen-anon-ns))))

;;;
;;; predicates
;;;
(libcore:intern :prelude "not" (:lambda (value) (prelude:null value)))
(libcore:intern :prelude "atom" (:lambda (value) (prelude:not (prelude:consp value))))

(libcore:intern :prelude "constantp"
   (:lambda (value)
      (:if (prelude:%orf (prelude:null value) (prelude:keywordp value))
           :t
           (:if (prelude:consp value)
                (libcore:eq :quote (libcore:car value))
                (prelude:not (prelude:symbolp value))))))

(libcore:intern :prelude "numberp"
  (:lambda (n)
     (:if (prelude:fixnump n)
          :t
          (prelude:floatp n))))

(libcore:intern :prelude "zerop"
   (:lambda (n)
     (:if (prelude:numberp n)
          (:if (libcore:eq 0 n)
               :t
               (libcore:eq 0.0 n))
          (prelude:raise n 'prelude:zerop "not a number"))))

(libcore:intern :prelude "minusp"
   (:lambda (n)
      (:if (prelude:numberp n)
           (:if (prelude:floatp n)
                (libcore:fl-lt n 0.0)
                (libcore:fx-lt n 0))
           (prelude:raise n 'prelude:minusp "not a number"))))

(libcore:intern :prelude "plusp"
   (:lambda (n)
      (:if (prelude:numberp n)
           (:if (prelude:floatp n)
                (libcore:fl-lt 0.0 n)
                (libcore:fx-lt 0 n))
           (prelude:raise n 'prelude:plusp "not a number"))))

;;;
;;; system classes
;;;
(libcore:intern :prelude "charp" (:lambda (t) (libcore:eq :char (libcore:type-of t))))
(libcore:intern :prelude "fixnump" (:lambda (t) (libcore:eq :fixnum (libcore:type-of t))))
(libcore:intern :prelude "floatp" (:lambda (t) (libcore:eq :float (libcore:type-of t))))
(libcore:intern :prelude "structp" (:lambda (t) (libcore:eq :struct (libcore:type-of t))))

;;;
;;; syntactic sugar
;;;
(libcore:intern :prelude "%orf"
   (:lambda (arg arg1)
     (:if arg arg arg1)))

(libcore:intern :prelude "%andf"
   (:lambda (arg arg1)
     (:if arg arg1 ())))

;;;
;;; apply
;;;
(libcore:intern :prelude "apply"
   (:lambda (function arg-list)
     (:if (prelude:functionp function)
          (:if (prelude:%prelude-function-p function)
               (prelude:%funct-apply function arg-list)
               (libcore:apply function arg-list))
          (prelude:raise function 'prelude:apply "not a function designator"))))

;;;
;;; namespace components
;;;
(libcore:intern :prelude "%gen-anon-ns"
   (:lambda ()
      (libcore:make-ns (prelude:genkeyword #\%))))
