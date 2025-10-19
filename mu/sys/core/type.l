;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core types
;;;
(mu:intern core "%type%" (mu:open :string :bidir "" :t))

(mu:intern core "%typep"
   (:lambda (value)
      (:if (core:structp value)
           (mu:eq :<type> (mu:struct-type value))
           ())))

;;;
;;; type struct: #s(:<type> #(:t name property-alist))
;;;
;;; where the property-alist is ((:property . accessor) ...)
;;;
(mu:intern core "%deftype"
   (:lambda (name properties)
     (:if (core:listp properties)
          ((:lambda (symbol)
             (:if (core:null symbol)
                  (mu:intern core:%types/ name (mu:make-struct :<type> (core:%list2 name properties)))
                  (mu:raise-from symbol 'core:%deftype :unbound)))
           (mu:find core:%types/ name))
          (mu:raise-from properties 'core:%deftype :type))))

(mu:intern core "%type-predicate"
   (:lambda (type value)
      ((:lambda (predicate)
          (mu:apply predicate (mu:cons value ())))
       (mu:cdr
        (core:assq
         type
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
            ,(mu:cons :vector (:lambda (t) (core:vectorp t)))))))))

(mu:intern core "%make-type"
   (:lambda (name property-values)
      ((:lambda (type-symbol)
         (core:mapc
          (:lambda (property-value)
            (core:find-if
             (:lambda (property-def)
               (:if (mu:eq (mu:car property-value) (mu:car property-def))
                        ((:lambda (type value)
                           (:if (core:%type-predicate type value)
                                property-def
                                (mu:raise-from property-value 'core:%make-type :type)))
                         (mu:cdr property-def)
                         (mu:cdr property-value))
                        ()))
             (mu:svref (mu:struct-vec (mu:symbol-value type-symbol)) 1)))
          property-values))
       (mu:find core:%types/ name))
     (mu:make-struct :<type> (core:%list2 name property-values))))

(mu:intern core "%type-prop"
   (:lambda (prop-name struct)
      (:if (core:%typep struct)
           (:if (core:keywordp prop-name)
                (core:assq prop-name (mu:svref (mu:struct-vec struct) 1))
                (mu:%raise struct 'core:%type-prop :type))
           (mu:raise-from prop-name 'core:%type-prop :range))))

;;;
;;; deftype deftype
;;;

;;;
;;;  (:name   . :symbolp)    name symbol
;;;  (:spec   . :t)          typespec form
;;;
(core:%deftype "%deftype"
    '((:name . :symbolp)
      (:spec . :t)))

(mu:intern core "%deftype-prop"
   (:lambda (prop deftype)
     (mu:cdr (core:%type-prop deftype))))

(mu:intern core "%make-deftype"
   (:lambda (name spec)
     (core:%make-type "%deftype"
        `(,(mu:cons :name name)
          ,(mu:cons :spec spec)))))

(mu:intern core "%deftypep"
   (:lambda (type)
     (:if (core:%typep type)
          (mu:eq '%deftype (core:type-of type))
          ())))

(mu:intern core "type-of"
   (:lambda (form)
     ((:lambda (type)
        (:if (mu:eq type :struct)
             (:if (mu:eq :<type> (mu:struct-type form))
                  (core:%prog2
                     (mu:write (mu:svref (mu:struct-vec form) 0) () core:%type%)
                     (mu:read core:%type% :t ()))
                  (mu:struct-type form))
             (:if (core:stringp form)
                  'string
                  (mu:cdr
                   (core:assq
                    type
                    '((:fixnum  . fixnum)
                      (:float   . float)
                      (:async   . async)
                      (:char    . char)
                      (:cons    . cons)
                      (:stream  . stream)
                      (:null    . symbol)
                      (:keyword . symbol)
                      (:symbol  . symbol)
                      (:vector  . vector)
                      (:func    . function)))))))
      (mu:type-of form))))
