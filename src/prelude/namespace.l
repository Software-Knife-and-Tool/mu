;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  prelude namespace
;;;

;;;
;;; namespace implementation
;;;
(libcore:intern :prelude "namespacep"
   (:lambda (ns)
      (libcore:eq (prelude:type-of ns) 'namespace)))

(libcore:intern :prelude "%make-namespc"
   (:lambda (name uses)
     (prelude:%make-type "namespc"
       `(,(libcore:cons :name name)
          ,(libcore:cons :intern (libcore:make-ns))
          ,(libcore:cons :extern (libcore:make-ns))
          ,(libcore:cons :ch-mac (libcore:make-ns))
          ,(libcore:cons :sy-mac (libcore:make-ns))
          ,(libcore:cons :uses uses)))))

(libcore:intern :prelude "%namespace-prop"
   (:lambda (prop namespace)
      (:if (prelude:findl
            (:lambda (lambda-prop)
               (libcore:eq prop lambda-prop))
            '(:name :intern :extern :ch-mac :rd-mac :sy-mac :uses))
           (prelude:%type-ref prop namespace)
           (prelude:raise prop 'prelude:%namespace-prop "not a namespace property"))))

;;;
;;; namespace functions
;;;
(libcore:intern :prelude "make-namespace"
   (:lambda (name uses)
      (:if (prelude:stringp name)
           (:if (prelude:listp uses)
                (prelude:%make-namespc name uses)
                (prelude:raise uses 'prelude:make-namespace "not a uses list"))
           (prelude:raise name 'prelude:make-namespace "not a name string"))))

(libcore:intern :prelude "intern"
   (:lambda (symbol value)          
      (:if (prelude:uninternedp symbol)
           ((:lambda (sym)
               (:if sym
                    (:if (prelude:boundp sym)
                         symbol
                         (libcore:intern (prelude:%reader-get-ns) (libcore:sy-name symbol) value))
                    (libcore:intern (prelude:%reader-get-ns) (libcore:sy-name symbol) value)))
            (libcore:ns-find (prelude:%reader-get-ns) (libcore:sy-name symbol)))
           ((:lambda (name)
               (libcore:intern
                (prelude:%reader-get-ns)
                name
                value))
            (libcore:sy-name symbol)))))
