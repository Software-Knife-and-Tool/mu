;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core types
;;;
(mu:intern core "%typep"
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
                  (mu:intern core:%types/ name (mu:make-struct :<core> (core:%list2 name properties)))
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
     (mu:make-struct :<core> (core:%list2 name property-values))))

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

;;;
;;; typep
;;;
(mu:intern core "%typespec-map%"
   `((bit-vector (satisfies core:bit-vector-p))
     (char (satisfies core:charp))
     (function (satisfies core:functionp))
     (map :map)
     (number (or (satisfies core:floatp) integer))
     (integer (satisfies core:fixnump))
     (sequence (or (satisfies core:vectorp) (satisfies core:stringp) (satisfies core:listp)))
     (stream (satisfies core:streamp))
     (string (satisfies core:stringp))
     (struct (satisifes core:structp))
     (symbol (or (satisfies core:symbolp) (satisfies core:keywordp)))))

(mu:intern core "%compound-typespec-map%"
   `(,(mu:cons 'satisfies
         (:lambda (value spec-list)
            (:if (mu:eq 1 (mu:length spec-list))
                 ((:lambda (predicate-symbol)
                    (:if (core:symbolp predicate-symbol)
                         (:if (mu:boundp predicate-symbol)
                              (core:apply (mu:symbol-value predicate-symbol) (mu:cons value ()))
                              (mu:raise-from predicate-symbol 'core:typep :range))
                         (mu:raise-from predicate-symbol 'core:typep :range)))
                  (mu:car spec-list))
                 (mu:raise-from spec-list 'core:typep :syntax))))
     ,(mu:cons 'not (:lambda (value spec-list)
                       (:if (mu:eq 1 (mu:length spec-list))
                            (core:null (core:typep value (mu:car spec-list)))
                            (mu:raise-from spec-list 'core:typep :syntax))))
     ,(mu:cons 'and (:lambda (value spec-list)
                       (:if (core:null spec-list)
                            :t
                            (core:foldl
                             (:lambda (elt acc)
                                (core:%and acc (core:typep value elt)))
                             :t
                             spec-list))))
     ,(mu:cons 'or (:lambda (value spec-list)
                     (:if (core:null spec-list)
                          ()
                          (core:foldl
                           (:lambda (elt acc)
                             (core:%or acc (core:typep value elt)))
                           :t
                           spec-list))))))

(mu:intern core "%typep"
   (:lambda (value typespec)
     (:if (core:keywordp typespec)
          (mu:eq typespec (mu:type-of value))
          (:if (core:%deftypep typespec)
               (core:typep value (core:%deftype-prop :spec typespec))
               (:if (core:symbolp typespec)
                    ((:lambda (spec-phrase)
                       (:if (core:null spec-phrase)
                            (mu:raise-from typespec 'core:typep :syntax)
                            (core:typep value (mu:car (mu:cdr spec-phrase)))))
                     (core:assq typespec core:%typespec-map%))
                    (:if (core:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (core:null spec-phrase)
                                    (mu:raise-from typespec 'core:typep :syntax)
                                    (mu:apply (mu:cdr spec-phrase) (core:%list2 value spec-list))))
                             (core:assq spec core:%compound-typespec-map%)))
                          (mu:car typespec)
                          (mu:cdr typespec))
                         (mu:raise-from typespec 'core:typep :synatx)))))))   

;;;
;;; types
;;;
(mu:intern core "%quote%" (mu:open :string :bidir "" :t))

(mu:intern core "typep"
   (:lambda (value typespec)
     (:if (core:keywordp typespec)
          (mu:eq typespec (mu:type-of value))
          (:if (core:%deftypep typespec)
               (core:typep value (core:%deftype-prop :spec typespec))
               (:if (core:symbolp typespec)
                    ((:lambda (spec-phrase)
                       (:if (core:null spec-phrase)
                            (mu:raise-from typespec 'core:typep :syntax)
                            (core:typep value (mu:car (mu:cdr spec-phrase)))))
                     (core:assq typespec core:%typespec-map%))
                    (:if (core:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (core:null spec-phrase)
                                    (mu:raise-from typespec 'core:typep :range)
                                    (mu:apply (mu:cdr spec-phrase) (core:%list2 value spec-list))))
                             (core:assq spec core:%compound-typespec-map%)))
                          (mu:car typespec)
                          (mu:cdr typespec))
                         (mu:raise-from typespec 'core:typep :syntax)))))))   

(mu:intern core "type-of"
   (:lambda (form)
     ((:lambda (type)
        (:if (mu:eq type :struct)
             (:if (mu:eq :<core> (mu:struct-type form))
                  (core:%prog2
                     (mu:write (mu:svref (mu:struct-vec form) 0) () core:%quote%)
                     (mu:read core:%quote% :t ()))
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
