;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; package deftype
;;;
(core:def-type "%package"
    '((:name    . :string)
      (:intern  . :keyword)
      (:extern  . :keyword)
      (:ch-mac  . :keyword)
      (:sy-mac  . :keyword)
      (:uses    . :listp)))

(mu:intern core "%package-prop"
   (:lambda (prop package)
      (:if (core:%findl-if
            (:lambda (lambda-prop)
               (mu:eq prop lambda-prop))
            '(:name :intern :extern :ch-mac :sy-mac :uses))
           (core:%type-ref prop package)
           (core:raise prop 'core:%package-prop "not a package property"))))

(mu:intern core "%make-package"
   (:lambda (name uses)
     (core:%make-core-type "%package"
       `(,(mu:cons :name name)
          ,(mu:cons :intern (mu:make-namespace))
          ,(mu:cons :extern (mu:make-namespace))
          ,(mu:cons :ch-mac (mu:make-namespace))
          ,(mu:cons :sy-mac (mu:make-namespace))
          ,(mu:cons :uses uses)))))

;;;
;;; predicate
;;;
(mu:intern common "packagep"
    (:lambda (ex)
      (core:logand
       (core:%core-type-p ex)
       (mu:eq '%exception (core:type-of ex)))))

;;;
;;; functions
;;;
(mu:intern common "intern"
   (:lambda (symbol value)          
      (:if (core:%uninternedp symbol)
           ((:lambda (sym)
               (:if sym
                    (:if (mu:boundp sym)
                         symbol
                         (mu:intern (core:%reader-get-ns) (mu:symbol-name symbol) value))
                    (mu:intern (core:%reader-get-ns) (mu:symbol-name symbol) value)))
            (mu:find (core:%reader-get-ns) (mu:symbol-name symbol)))
           ((:lambda (name)
               (mu:intern
                (core:%reader-get-ns)
                name
                value))
            (mu:symbol-name symbol)))))
