;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude types
;;;
(libcore:intern :prelude "%types-ns" (libcore:make-ns :%types))

(libcore:intern :prelude "%prelude-type-p"
   (:lambda (value)
      (:if (prelude:structp value)
           (libcore:eq :<type> (libcore:st-type value))
           ())))

;;;
;;; type struct: #s(:<type> #(:t name property-alist))
;;;
;;; where the property-alist is ((:predicate . :predicate) ...)
;;;
(libcore:intern :prelude "define-type"
   (:lambda (name properties)
      ((:lambda (str)
          (:if (prelude:listp properties)
               ((:lambda (symbol)
                   (:if (prelude:null symbol)
                        (libcore:intern :%types str (libcore:struct :<type> `(,str ,properties)))
                        (prelude:raise symbol 'prelude:define-type "type symbol bound")))
                (libcore:ns-find :%types str))
               (prelude:raise properties 'prelude:define-type "not a property list")))
       (prelude:string name))))

(libcore:intern :prelude "%type-predicate-map%"
   `(,(libcore:cons :char (:lambda (t) (prelude:charp t)))
      ,(libcore:cons :cons (:lambda (t) (prelude:consp t)))
      ,(libcore:cons :fixnum (:lambda (t) (prelude:fixnump t)))
      ,(libcore:cons :float (:lambda (t) (prelude:floatp t)))
      ,(libcore:cons :func (:lambda (t) (prelude:functionp t)))
      ,(libcore:cons :listp (:lambda (t) (prelude:listp t)))
      ,(libcore:cons :map (:lambda (t) (libcore:eq :map (libcore:type-of t))))
      ,(libcore:cons :keyword (:lambda (t) (prelude:keywordp t)))
      ,(libcore:cons :namespc (:lambda (t) (prelude:namespacep t)))
      ,(libcore:cons :string (:lambda (t) (prelude:stringp t)))
      ,(libcore:cons :struct (:lambda (t) (prelude:structp t)))
      ,(libcore:cons :symbol (:lambda (t) (libcore:eq :symbol (libcore:type-of t))))
      ,(libcore:cons :symbolp (:lambda (t) (prelude:symbolp t)))
      ,(libcore:cons :t (:lambda (t) :t))
      ,(libcore:cons :vector (:lambda (t) (prelude:vectorp t)))))

(libcore:intern :prelude "%type-predicate"
   (:lambda (type value)
      ((:lambda (predicate)
          (libcore:apply predicate `(,value)))
       (libcore:cdr (prelude:%assoc type prelude:%type-predicate-map%)))))

(libcore:intern :prelude "%make-type"
    (:lambda (name property-values)
       ((:lambda (type-symbol)
           (prelude:%mapc
            (:lambda (property-value)
               (prelude:findl
                (:lambda (property-def)
                   (:if (libcore:eq (libcore:car property-value) (libcore:car property-def))
                        ((:lambda (type value)
                            (:if (prelude:%type-predicate type value)
                                 property-def
                                 (prelude:raise property-value '%make-type "type mismatch")))
                         (libcore:cdr property-def)
                         (libcore:cdr property-value))
                        ()))
                (libcore:sv-ref (libcore:st-vec (libcore:sy-val type-symbol)) 1)))
            property-values))
        (libcore:ns-find prelude:%types-ns name))
       (libcore:struct :<type> `(,name ,property-values))))

;;;
;;; returns () if not found
;;;
(libcore:intern :prelude "%type-ref"
   (:lambda (prop-name struct)
      (:if (prelude:%prelude-type-p struct)
           (:if (prelude:keywordp prop-name)
                (prelude:%assoc prop-name (libcore:sv-ref (libcore:st-vec struct) 1))
                (prelude:raise struct 'prelude:%type-ref "not a prelude type"))
           (prelude:raise prop-name 'prelude:%type-ref "not a property name"))))
