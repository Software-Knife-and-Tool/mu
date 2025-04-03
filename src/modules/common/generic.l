;;;  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; deftype generic
;;;

;;;
;;;  (:name   . :symbolp)    name symbol
;;;  (:spec   . :t)          typespec form
;;;
(core:%deftype "%generic"
    '((:name . :symbolp)
      (:spec . :t)))

(mu:intern common "%generic-prop"
   (:lambda (prop deftype)
     (mu:cdr (core:%type-ref generic))))

(mu:intern common "%make-generic"
   (:lambda (name spec)
     (core:%make-core-type "%generic"
        `(,(mu:cons :name name)
          ,(mu:cons :spec spec)))))

(mu:intern common "%deftypep"
   (:lambda (type)
     (:if (core:%core-type-p type)
          (mu:eq '%generic (core:type-of type))
          ())))

(defmacro defgeneric (name lambda &rest body)
  (mu:intern
   (mu:symbol-namespace)
   (mu:symbol-name)
   (common:%make-generic name `(mu:apply (:lambda () ,@body) ())))
  `(:quote ,@name))
