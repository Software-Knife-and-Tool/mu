;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  prelude namespace
;;;

;;;
;;; namespace implementation
;;;
(crux:intern prelude "namespacep"
   (:lambda (ns)
      (crux:eq (prelude:type-of ns) 'namespace)))

(crux:intern prelude "%make-namespace"
   (:lambda (name uses)
     (prelude:%make-type "%ns"
       `(,(crux:cons :name name)
          ,(crux:cons :intern (crux:make-ns))
          ,(crux:cons :extern (crux:make-ns))
          ,(crux:cons :ch-mac (crux:make-ns))
          ,(crux:cons :sy-mac (crux:make-ns))
          ,(crux:cons :uses uses)))))

(crux:intern prelude "%namespace-prop"
   (:lambda (prop namespace)
      (:if (prelude:findl
            (:lambda (lambda-prop)
               (crux:eq prop lambda-prop))
            '(:name :intern :extern :ch-mac :rd-mac :sy-mac :uses))
           (prelude:%type-ref prop namespace)
           (prelude:raise prop 'prelude:%namespace-prop "not a namespace property"))))

;;;
;;; namespace functions
;;;
(crux:intern prelude "make-namespace"
   (:lambda (name uses)
      (:if (prelude:stringp name)
           (:if (prelude:listp uses)
                (prelude:%make-namespace name uses)
                (prelude:raise uses 'prelude:make-namespace "not a uses list"))
           (prelude:raise name 'prelude:make-namespace "not a name string"))))

(crux:intern prelude "intern"
   (:lambda (symbol value)          
      (:if (prelude:uninternedp symbol)
           ((:lambda (sym)
               (:if sym
                    (:if (prelude:boundp sym)
                         symbol
                         (crux:intern (prelude:%reader-get-ns) (crux:symbol-name symbol) value))
                    (crux:intern (prelude:%reader-get-ns) (crux:symbol-name symbol) value)))
            (crux:find (prelude:%reader-get-ns) (crux:symbol-name symbol)))
           ((:lambda (name)
               (crux:intern
                (prelude:%reader-get-ns)
                name
                value))
            (crux:symbol-name symbol)))))
