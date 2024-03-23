;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  prelude namespace
;;;

;;;
;;; namespace implementation
;;;
(core:intern :prelude "namespacep"
   (:lambda (ns)
      (core:eq (prelude:type-of ns) 'namespace)))

(core:intern :prelude "%make-namespc"
   (:lambda (name uses)
     (prelude:%make-type "namespc"
       `(,(core:cons :name name)
          ,(core:cons :intern (core:make-ns))
          ,(core:cons :extern (core:make-ns))
          ,(core:cons :ch-mac (core:make-ns))
          ,(core:cons :sy-mac (core:make-ns))
          ,(core:cons :uses uses)))))

(core:intern :prelude "%namespace-prop"
   (:lambda (prop namespace)
      (:if (prelude:findl
            (:lambda (lambda-prop)
               (core:eq prop lambda-prop))
            '(:name :intern :extern :ch-mac :rd-mac :sy-mac :uses))
           (prelude:%type-ref prop namespace)
           (prelude:raise prop 'prelude:%namespace-prop "not a namespace property"))))

;;;
;;; namespace functions
;;;
(core:intern :prelude "make-namespace"
   (:lambda (name uses)
      (:if (prelude:stringp name)
           (:if (prelude:listp uses)
                (prelude:%make-namespc name uses)
                (prelude:raise uses 'prelude:make-namespace "not a uses list"))
           (prelude:raise name 'prelude:make-namespace "not a name string"))))

(core:intern :prelude "intern"
   (:lambda (symbol value)          
      (:if (prelude:uninternedp symbol)
           ((:lambda (sym)
               (:if sym
                    (:if (prelude:boundp sym)
                         symbol
                         (core:intern (prelude:%reader-get-ns) (core:sy-name symbol) value))
                    (core:intern (prelude:%reader-get-ns) (core:sy-name symbol) value)))
            (core:ns-find (prelude:%reader-get-ns) (core:sy-name symbol)))
           ((:lambda (name)
               (core:intern
                (prelude:%reader-get-ns)
                name
                value))
            (core:sy-name symbol)))))
