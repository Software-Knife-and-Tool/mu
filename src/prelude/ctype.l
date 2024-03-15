;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude types
;;;
(mu:intern :prelude "%types-ns" (mu:make-ns :%types))

(mu:intern :prelude "%prelude-type-p"
   (:lambda (value)
      (:if (prelude:structp value)
           (mu:eq :<type> (mu:st-type value))
           ())))

;;;
;;; type struct: #s(:<type> #(:t name property-alist))
;;;
;;; where the property-alist is ((:predicate . :predicate) ...)
;;;
(mu:intern :prelude "define-type"
   (:lambda (name properties)
      ((:lambda (str)
          (:if (prelude:listp properties)
               ((:lambda (symbol)
                   (:if (prelude:null symbol)
                        (mu:intern :%types str (mu:struct :<type> `(,str ,properties)))
                        (prelude:raise symbol 'prelude:define-type "type symbol bound")))
                (mu:ns-find :%types str))
               (prelude:raise properties 'prelude:define-type "not a property list")))
       (prelude:string name))))

(mu:intern :prelude "%type-predicate-map%"
   `(,(mu:cons :char (:lambda (t) (prelude:charp t)))
      ,(mu:cons :cons (:lambda (t) (prelude:consp t)))
      ,(mu:cons :fixnum (:lambda (t) (prelude:fixnump t)))
      ,(mu:cons :float (:lambda (t) (prelude:floatp t)))
      ,(mu:cons :func (:lambda (t) (prelude:functionp t)))
      ,(mu:cons :listp (:lambda (t) (prelude:listp t)))
      ,(mu:cons :map (:lambda (t) (mu:eq :map (mu:type-of t))))
      ,(mu:cons :keyword (:lambda (t) (prelude:keywordp t)))
      ,(mu:cons :namespc (:lambda (t) (prelude:namespacep t)))
      ,(mu:cons :string (:lambda (t) (prelude:stringp t)))
      ,(mu:cons :struct (:lambda (t) (prelude:structp t)))
      ,(mu:cons :symbol (:lambda (t) (mu:eq :symbol (mu:type-of t))))
      ,(mu:cons :symbolp (:lambda (t) (prelude:symbolp t)))
      ,(mu:cons :t (:lambda (t) :t))
      ,(mu:cons :vector (:lambda (t) (prelude:vectorp t)))))

(mu:intern :prelude "%type-predicate"
   (:lambda (type value)
      ((:lambda (predicate)
          (mu:apply predicate `(,value)))
       (mu:cdr (prelude:%assoc type prelude:%type-predicate-map%)))))

(mu:intern :prelude "%make-type"
    (:lambda (name property-values)
       ((:lambda (type-symbol)
           (prelude:%mapc
            (:lambda (property-value)
               (prelude:findl
                (:lambda (property-def)
                   (:if (mu:eq (mu:car property-value) (mu:car property-def))
                        ((:lambda (type value)
                            (:if (prelude:%type-predicate type value)
                                 property-def
                                 (prelude:raise property-value '%make-type "type mismatch")))
                         (mu:cdr property-def)
                         (mu:cdr property-value))
                        ()))
                (mu:sv-ref (mu:st-vec (mu:sy-val type-symbol)) 1)))
            property-values))
        (mu:ns-find prelude:%types-ns name))
       (mu:struct :<type> `(,name ,property-values))))

;;;
;;; returns () if not found
;;;
(mu:intern :prelude "%type-ref"
   (:lambda (prop-name struct)
      (:if (prelude:%prelude-type-p struct)
           (:if (prelude:keywordp prop-name)
                (prelude:%assoc prop-name (mu:sv-ref (mu:st-vec struct) 1))
                (prelude:raise struct 'prelude:%type-ref "not a prelude type"))
           (prelude:raise prop-name 'prelude:%type-ref "not a property name"))))
