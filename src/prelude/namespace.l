;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  prelude namespace
;;;

;;;
;;; namespace implementation
;;;
(lib:intern prelude "namespacep"
   (:lambda (ns)
      (lib:eq (prelude:type-of ns) 'namespace)))

(lib:intern prelude "%make-namespc"
   (:lambda (name uses)
     (prelude:%make-type "namespc"
       `(,(lib:cons :name name)
          ,(lib:cons :intern (lib:make-ns))
          ,(lib:cons :extern (lib:make-ns))
          ,(lib:cons :ch-mac (lib:make-ns))
          ,(lib:cons :sy-mac (lib:make-ns))
          ,(lib:cons :uses uses)))))

(lib:intern prelude "%namespace-prop"
   (:lambda (prop namespace)
      (:if (prelude:findl
            (:lambda (lambda-prop)
               (lib:eq prop lambda-prop))
            '(:name :intern :extern :ch-mac :rd-mac :sy-mac :uses))
           (prelude:%type-ref prop namespace)
           (prelude:raise prop 'prelude:%namespace-prop "not a namespace property"))))

;;;
;;; namespace functions
;;;
(lib:intern prelude "make-namespace"
   (:lambda (name uses)
      (:if (prelude:stringp name)
           (:if (prelude:listp uses)
                (prelude:%make-namespc name uses)
                (prelude:raise uses 'prelude:make-namespace "not a uses list"))
           (prelude:raise name 'prelude:make-namespace "not a name string"))))

(lib:intern prelude "intern"
   (:lambda (symbol value)          
      (:if (prelude:uninternedp symbol)
           ((:lambda (sym)
               (:if sym
                    (:if (prelude:boundp sym)
                         symbol
                         (lib:intern (prelude:%reader-get-ns) (lib:symbol-name symbol) value))
                    (lib:intern (prelude:%reader-get-ns) (lib:symbol-name symbol) value)))
            (lib:find (prelude:%reader-get-ns) (lib:symbol-name symbol)))
           ((:lambda (name)
               (lib:intern
                (prelude:%reader-get-ns)
                name
                value))
            (lib:symbol-name symbol)))))
