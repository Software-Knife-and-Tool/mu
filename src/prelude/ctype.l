;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude types
;;;
(lib:intern :prelude "%types-ns" (lib:make-ns "%types"))

(lib:intern :prelude "%prelude-type-p"
   (:lambda (value)
      (:if (prelude:structp value)
           (lib:eq :<type> (lib:st-type value))
           ())))

;;;
;;; type struct: #s(:<type> #(:t name property-alist))
;;;
;;; where the property-alist is ((:predicate . :predicate) ...)
;;;
(lib:intern :prelude "define-type"
   (:lambda (name properties)
      ((:lambda (str)
          (:if (prelude:listp properties)
               ((:lambda (symbol)
                   (:if (prelude:null symbol)
                        (lib:intern :%types str (lib:struct :<type> `(,str ,properties)))
                        (prelude:raise symbol 'prelude:define-type "type symbol bound")))
                (lib:ns-find :%types str))
               (prelude:raise properties 'prelude:define-type "not a property list")))
       (prelude:string name))))

(lib:intern :prelude "%type-predicate-map%"
   `(,(lib:cons :char (:lambda (t) (prelude:charp t)))
      ,(lib:cons :cons (:lambda (t) (prelude:consp t)))
      ,(lib:cons :fixnum (:lambda (t) (prelude:fixnump t)))
      ,(lib:cons :float (:lambda (t) (prelude:floatp t)))
      ,(lib:cons :func (:lambda (t) (prelude:functionp t)))
      ,(lib:cons :listp (:lambda (t) (prelude:listp t)))
      ,(lib:cons :map (:lambda (t) (lib:eq :map (lib:type-of t))))
      ,(lib:cons :keyword (:lambda (t) (prelude:keywordp t)))
      ,(lib:cons :namespc (:lambda (t) (prelude:namespacep t)))
      ,(lib:cons :string (:lambda (t) (prelude:stringp t)))
      ,(lib:cons :struct (:lambda (t) (prelude:structp t)))
      ,(lib:cons :symbol (:lambda (t) (lib:eq :symbol (lib:type-of t))))
      ,(lib:cons :symbolp (:lambda (t) (prelude:symbolp t)))
      ,(lib:cons :t (:lambda (t) :t))
      ,(lib:cons :vector (:lambda (t) (prelude:vectorp t)))))

(lib:intern :prelude "%type-predicate"
   (:lambda (type value)
      ((:lambda (predicate)
          (lib:apply predicate `(,value)))
       (lib:cdr (prelude:%assoc type prelude:%type-predicate-map%)))))

(lib:intern :prelude "%make-type"
    (:lambda (name property-values)
       ((:lambda (type-symbol)
           (prelude:%mapc
            (:lambda (property-value)
               (prelude:findl
                (:lambda (property-def)
                   (:if (lib:eq (lib:car property-value) (lib:car property-def))
                        ((:lambda (type value)
                            (:if (prelude:%type-predicate type value)
                                 property-def
                                 (prelude:raise property-value '%make-type "type mismatch")))
                         (lib:cdr property-def)
                         (lib:cdr property-value))
                        ()))
                (lib:sv-ref (lib:st-vec (lib:sy-val type-symbol)) 1)))
            property-values))
        (lib:ns-find prelude:%types-ns name))
       (lib:struct :<type> `(,name ,property-values))))

;;;
;;; returns () if not found
;;;
(lib:intern :prelude "%type-ref"
   (:lambda (prop-name struct)
      (:if (prelude:%prelude-type-p struct)
           (:if (prelude:keywordp prop-name)
                (prelude:%assoc prop-name (lib:sv-ref (lib:st-vec struct) 1))
                (prelude:raise struct 'prelude:%type-ref "not a prelude type"))
           (prelude:raise prop-name 'prelude:%type-ref "not a property name"))))
