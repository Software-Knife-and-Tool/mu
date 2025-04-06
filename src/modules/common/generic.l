;;;  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; deftype generic
;;;

;;;
;;;  (:name   . :symbolp)    name symbol
;;;  (:impls  . :listp)      implementation alist
;;;     '((typespec . function) ...)
;;;
(core:%deftype "%generic"
    '((:name  . :symbolp)
      (:impls . :listp)))

(mu:intern common "%generic-prop"
   (:lambda (prop generic)
      (:if (core:%findl-if
            (:lambda (generic-prop)
               (mu:eq prop generic-prop))
            '(:name :impls))
           (core:%type-ref prop generic)
           (core:raise prop 'core:%generic-prop "not a generic property"))))

(mu:intern common "%make-generic"
   (:lambda (name impls)
     (core:%make-core-type "%generic"
        `(,(mu:cons :name name)
          ,(mu:cons :impls impls)))))

(mu:intern common "%genericp"
   (:lambda (type)
     (:if (core:%core-type-p type)
          (mu:eq '%generic (core:type-of type))
          ())))

(defmacro %defgeneric (symbol &rest body)
  (mu:intern
   (mu:symbol-namespace symbol)
   (mu:symbol-name symbol)
   (common:%make-generic
    symbol
    (core:%mapcar
     (:lambda (impl)
       ((:lambda (typespec lambda)
          (mu:cons typespec (mu:compile lambda)))
        (mu:car impl)
        (mu:cdr impl)))
       body)))
  `(:quote ,@symbol))

(mu:intern common "%apply-generic"
   (:lambda (generic args)
     (:if (common:%genericp generic)
          ((:lambda (object impls)
             ((:lambda (type-function)
                (:if type-function
                     (core:apply type-function args)
                     (core:raise type-function 'common:%apply-generic "not a type function")))
              (mu:cdr (core:%findl-if (:lambda (impl) (core:warn impl "impl") (common:typep object (mu:car impl))) impls))))
           (mu:car args)
           (mu:cdr (common:%generic-prop :impls generic)))
          (core:raise generic 'common:%apply-generic "invalid argument"))))
