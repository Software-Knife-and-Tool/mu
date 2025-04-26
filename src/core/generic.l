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

(mu:intern core "%generic-prop"
   (:lambda (prop generic)
      (:if (core:%find-if
            (:lambda (generic-prop)
               (mu:eq prop generic-prop))
            '(:name :impls))
           (core:%type-prop prop generic)
           (core:%raise prop 'core:%generic-prop "not a generic property"))))

(mu:intern core "%make-generic"
   (:lambda (name impls)
     (core:%make-type "%generic"
        `(,(mu:cons :name name)
          ,(mu:cons :impls impls)))))

(mu:intern core "%genericp"
   (:lambda (type)
     (:if (core:%typep type)
          (mu:eq '%generic (core:type-of type))
          ())))

(mu:intern core "%apply-generic"
   (:lambda (generic args)
     (:if (core:%genericp generic)
          ((:lambda (object impls)
             ((:lambda (type-function)
                (:if type-function
                     (core:apply type-function args)
                     (core:%raise type-function 'core:%apply-generic "not a type function")))
              (mu:cdr (core:%find-if (:lambda (impl) (core:typep object (mu:car impl))) impls))))
           (mu:car args)
           (mu:cdr (core:%generic-prop :impls generic)))
          (core:%raise generic 'core:%apply-generic "invalid argument"))))
