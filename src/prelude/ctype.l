;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude types
;;;
(core:intern :prelude "%types-ns" (core:make-ns :%types))

(core:intern :prelude "%prelude-type-p"
   (:lambda (value)
      (:if (prelude:structp value)
           (core:eq :<type> (core:st-type value))
           ())))

;;;
;;; type struct: #s(:<type> #(:t name property-alist))
;;;
;;; where the property-alist is ((:predicate . :predicate) ...)
;;;
(core:intern :prelude "define-type"
   (:lambda (name properties)
      ((:lambda (str)
          (:if (prelude:listp properties)
               ((:lambda (symbol)
                   (:if (prelude:null symbol)
                        (core:intern :%types str (core:struct :<type> `(,str ,properties)))
                        (prelude:raise symbol 'prelude:define-type "type symbol bound")))
                (core:ns-find :%types str))
               (prelude:raise properties 'prelude:define-type "not a property list")))
       (prelude:string name))))

(core:intern :prelude "%type-predicate-map%"
   `(,(core:cons :char (:lambda (t) (prelude:charp t)))
      ,(core:cons :cons (:lambda (t) (prelude:consp t)))
      ,(core:cons :fixnum (:lambda (t) (prelude:fixnump t)))
      ,(core:cons :float (:lambda (t) (prelude:floatp t)))
      ,(core:cons :func (:lambda (t) (prelude:functionp t)))
      ,(core:cons :listp (:lambda (t) (prelude:listp t)))
      ,(core:cons :map (:lambda (t) (core:eq :map (core:type-of t))))
      ,(core:cons :keyword (:lambda (t) (prelude:keywordp t)))
      ,(core:cons :namespc (:lambda (t) (prelude:namespacep t)))
      ,(core:cons :string (:lambda (t) (prelude:stringp t)))
      ,(core:cons :struct (:lambda (t) (prelude:structp t)))
      ,(core:cons :symbol (:lambda (t) (core:eq :symbol (core:type-of t))))
      ,(core:cons :symbolp (:lambda (t) (prelude:symbolp t)))
      ,(core:cons :t (:lambda (t) :t))
      ,(core:cons :vector (:lambda (t) (prelude:vectorp t)))))

(core:intern :prelude "%type-predicate"
   (:lambda (type value)
      ((:lambda (predicate)
          (core:apply predicate `(,value)))
       (core:cdr (prelude:%assoc type prelude:%type-predicate-map%)))))

(core:intern :prelude "%make-type"
    (:lambda (name property-values)
       ((:lambda (type-symbol)
           (prelude:%mapc
            (:lambda (property-value)
               (prelude:findl
                (:lambda (property-def)
                   (:if (core:eq (core:car property-value) (core:car property-def))
                        ((:lambda (type value)
                            (:if (prelude:%type-predicate type value)
                                 property-def
                                 (prelude:raise property-value '%make-type "type mismatch")))
                         (core:cdr property-def)
                         (core:cdr property-value))
                        ()))
                (core:sv-ref (core:st-vec (core:sy-val type-symbol)) 1)))
            property-values))
        (core:ns-find prelude:%types-ns name))
       (core:struct :<type> `(,name ,property-values))))

;;;
;;; returns () if not found
;;;
(core:intern :prelude "%type-ref"
   (:lambda (prop-name struct)
      (:if (prelude:%prelude-type-p struct)
           (:if (prelude:keywordp prop-name)
                (prelude:%assoc prop-name (core:sv-ref (core:st-vec struct) 1))
                (prelude:raise struct 'prelude:%type-ref "not a prelude type"))
           (prelude:raise prop-name 'prelude:%type-ref "not a property name"))))
