;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  core namespace
;;;

;;;
;;; namespace type
;;;
(core:define-type "%ns"
    '((:name    . :string)
      (:intern  . :keyword)
      (:extern  . :keyword)
      (:ch-mac  . :keyword)
      (:sy-mac  . :keyword)
      (:uses    . :listp)))

(mu:intern core "%namespace-prop"
   (:lambda (prop namespace)
      (:if (core:findl
            (:lambda (lambda-prop)
               (mu:eq prop lambda-prop))
            '(:name :intern :extern :ch-mac :rd-mac :sy-mac :uses))
           (core:%type-ref prop namespace)
           (core:raise prop 'core:%namespace-prop "not a namespace property"))))

(mu:intern core "%make-namespace"
   (:lambda (name uses)
     (core:%make-type "%ns"
       `(,(mu:cons :name name)
          ,(mu:cons :intern (mu:make-namespace))
          ,(mu:cons :extern (mu:make-namespace))
          ,(mu:cons :ch-mac (mu:make-namespace))
          ,(mu:cons :sy-mac (mu:make-namespace))
          ,(mu:cons :uses uses)))))

;;;
;;; functions
;;;
(mu:intern core "make-namespace"
   (:lambda (name uses)
      (:if (core:stringp name)
           (:if (core:listp uses)
                (core:%make-namespace name uses)
                (core:raise uses 'core:make-namespace "not a uses list"))
           (core:raise name 'core:make-namespace "not a name string"))))

(mu:intern core "intern"
   (:lambda (symbol value)          
      (:if (core:uninternedp symbol)
           ((:lambda (sym)
               (:if sym
                    (:if (core:boundp sym)
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

(mu:intern core "%gen-anon-ns"
   (:lambda ()
      (mu:make-namespace (mu:symbol-name (core:genkeyword #\%)))))
