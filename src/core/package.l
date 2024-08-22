;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  core package
;;;

;;;
;;; package type
;;;
(core:define-type "%packge"
    '((:name    . :string)
      (:intern  . :keyword)
      (:extern  . :keyword)
      (:ch-mac  . :keyword)
      (:sy-mac  . :keyword)
      (:uses    . :listp)))

(mu:intern core "%package-prop"
   (:lambda (prop package)
      (:if (core:findl
            (:lambda (lambda-prop)
               (mu:eq prop lambda-prop))
            '(:name :intern :extern :ch-mac :rd-mac :sy-mac :uses))
           (core:%type-ref prop package)
           (core:raise prop 'core:%package-prop "not a package property"))))

(mu:intern core "%make-package"
   (:lambda (name uses)
     (core:%make-type "%packge"
       `(,(mu:cons :name name)
          ,(mu:cons :intern (mu:make-package))
          ,(mu:cons :extern (mu:make-package))
          ,(mu:cons :ch-mac (mu:make-package))
          ,(mu:cons :sy-mac (mu:make-package))
          ,(mu:cons :uses uses)))))

;;;
;;; functions
;;;
(mu:intern core "make-package"
   (:lambda (name uses)
      (:if (core:stringp name)
           (:if (core:listp uses)
                (core:%make-package name uses)
                (core:raise uses 'core:make-package "not a uses list"))
           (core:raise name 'core:make-package "not a name string"))))

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
