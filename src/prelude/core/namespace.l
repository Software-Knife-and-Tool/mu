;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  prelude namespace
;;;

;;;
;;; namespace implementation
;;;
(mu:intern prelude "namespacep"
   (:lambda (ns)
      (mu:eq (prelude:type-of ns) 'namespace)))

(mu:intern prelude "%make-namespace"
   (:lambda (name uses)
     (prelude:%make-type "%ns"
       `(,(mu:cons :name name)
          ,(mu:cons :intern (mu:make-namespace))
          ,(mu:cons :extern (mu:make-namespace))
          ,(mu:cons :ch-mac (mu:make-namespace))
          ,(mu:cons :sy-mac (mu:make-namespace))
          ,(mu:cons :uses uses)))))

(mu:intern prelude "%namespace-prop"
   (:lambda (prop namespace)
      (:if (prelude:findl
            (:lambda (lambda-prop)
               (mu:eq prop lambda-prop))
            '(:name :intern :extern :ch-mac :rd-mac :sy-mac :uses))
           (prelude:%type-ref prop namespace)
           (prelude:raise prop 'prelude:%namespace-prop "not a namespace property"))))

;;;
;;; namespace functions
;;;
(mu:intern prelude "make-namespace"
   (:lambda (name uses)
      (:if (prelude:stringp name)
           (:if (prelude:listp uses)
                (prelude:%make-namespace name uses)
                (prelude:raise uses 'prelude:make-namespace "not a uses list"))
           (prelude:raise name 'prelude:make-namespace "not a name string"))))

(mu:intern prelude "intern"
   (:lambda (symbol value)          
      (:if (prelude:uninternedp symbol)
           ((:lambda (sym)
               (:if sym
                    (:if (prelude:boundp sym)
                         symbol
                         (mu:intern (prelude:%reader-get-ns) (mu:symbol-name symbol) value))
                    (mu:intern (prelude:%reader-get-ns) (mu:symbol-name symbol) value)))
            (mu:find (prelude:%reader-get-ns) (mu:symbol-name symbol)))
           ((:lambda (name)
               (mu:intern
                (prelude:%reader-get-ns)
                name
                value))
            (mu:symbol-name symbol)))))
