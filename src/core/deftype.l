;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core types
;;;
(mu:intern core "%core-type-p"
   (:lambda (value)
      (:if (core:structp value)
           (mu:eq :<core> (mu:struct-type value))
           ())))

;;;
;;; type struct: #s(:<core> #(:t name property-alist))
;;;
;;; where the property-alist is ((:property . accessor) ...)
;;;
(mu:intern core "%deftype"
   (:lambda (name properties)
     (:if (core:listp properties)
          ((:lambda (symbol)
             (:if (core:null symbol)
                  (mu:intern core:%types-ns% name (mu:make-struct :<core> (core:%list2 name properties)))
                  (core:raise symbol 'core:%deftype "type symbol bound")))
           (mu:find core:%types-ns% name))
          (core:raise properties 'core:%deftype "not a property list"))))

(mu:intern core "%type-predicate-map%"
   `(,(mu:cons :char core:charp)
      ,(mu:cons :cons core:consp)
      ,(mu:cons :fixnum core:fixnump)
      ,(mu:cons :float core:floatp)
      ,(mu:cons :func (:lambda (t) (core:functionp t)))
      ,(mu:cons :listp core:listp)
      ,(mu:cons :keyword core:keywordp)
      ,(mu:cons :ns core:namespacep)
      ,(mu:cons :string (:lambda (t) (core:stringp t)))
      ,(mu:cons :struct core:structp)
      ,(mu:cons :symbol (:lambda (t) (mu:eq :symbol (mu:type-of t))))
      ,(mu:cons :symbolp (:lambda (t) (core:symbolp t)))
      ,(mu:cons :t (:lambda (t) :t))
      ,(mu:cons :vector (:lambda (t) (core:vectorp t)))))

(mu:intern core "%type-predicate"
   (:lambda (type value)
      ((:lambda (predicate)
          (mu:apply predicate (mu:cons value ())))
       (mu:cdr (core:%assq type core:%type-predicate-map%)))))

(mu:intern core "%make-core-type"
   (:lambda (name property-values)
      ((:lambda (type-symbol)
         (core:%mapc
          (:lambda (property-value)
            (core:%findl-if
             (:lambda (property-def)
               (:if (mu:eq (mu:car property-value) (mu:car property-def))
                        ((:lambda (type value)
                           (:if (core:%type-predicate type value)
                                property-def
                                (core:raise property-value 'core:%make-core-type "type mismatch")))
                         (mu:cdr property-def)
                         (mu:cdr property-value))
                        ()))
             (mu:svref (mu:struct-vec (mu:symbol-value type-symbol)) 1)))
          property-values))
       (mu:find core:%types-ns% name))
     (mu:make-struct :<core> (core:%list2 name property-values))))

(mu:intern core "%type-ref"
   (:lambda (prop-name struct)
      (:if (core:%core-type-p struct)
           (:if (core:keywordp prop-name)
                (core:%assq prop-name (mu:svref (mu:struct-vec struct) 1))
                (core:raise struct 'core:%type-ref "not a core type"))
           (core:raise prop-name 'core:%type-ref "not a property name"))))

;;;
;;; types
;;;
(mu:intern core "%quote%" (mu:open :string :bidir "" :t))

(mu:intern core "type-of"
   (:lambda (value)
      ((:lambda (type)
          (:if (mu:eq type :struct)
               (:if (mu:eq :<core> (mu:struct-type value))
                    (mu:intern mu:*null/* (mu:svref (mu:struct-vec value) 0) ())
                    (mu:struct-type value))
               (:if (core:stringp value)
                    'string
                    ((:lambda (symbol)
                       (:if symbol
                            symbol
                            ((:lambda ()
                               (mu:write (mu:symbol-name type) () core:%quote%)
                               (mu:read core:%quote% :t ())))))
                     (mu:find mu:*null/* (mu:symbol-name type))))))
       (mu:type-of value))))
