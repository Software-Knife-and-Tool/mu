;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude types
;;;
(mu:intern prelude "%types-ns" (mu:make-namespace "%types"))

(mu:intern prelude "%prelude-type-p"
   (:lambda (value)
      (:if (prelude:structp value)
           (mu:eq :<type> (mu:struct-type value))
           ())))

;;;
;;; type struct: #s(:<type> #(:t name property-alist))
;;;
;;; where the property-alist is ((:property . accessor) ...)
;;;
(mu:intern prelude "define-type"
   (:lambda (name properties)
      ((:lambda (str)
          (:if (prelude:listp properties)
               ((:lambda (symbol)
                   (:if (prelude:null symbol)
                        (mu:intern (mu:find-namespace "%types") str (mu:make-struct :<type> `(,str ,properties)))
                        (prelude:raise symbol 'prelude:define-type "type symbol bound")))
                (mu:find (mu:find-namespace "%types") str))
               (prelude:raise properties 'prelude:define-type "not a property list")))
       (prelude:string name))))

(mu:intern prelude "%type-predicate-map%"
   `(,(mu:cons :char (:lambda (t) (prelude:charp t)))
      ,(mu:cons :cons (:lambda (t) (prelude:consp t)))
      ,(mu:cons :fixnum (:lambda (t) (prelude:fixnump t)))
      ,(mu:cons :float (:lambda (t) (prelude:floatp t)))
      ,(mu:cons :func (:lambda (t) (prelude:functionp t)))
      ,(mu:cons :listp (:lambda (t) (prelude:listp t)))
      ,(mu:cons :map (:lambda (t) (mu:eq :map (mu:type-of t))))
      ,(mu:cons :keyword (:lambda (t) (prelude:keywordp t)))
      ,(mu:cons :ns (:lambda (t) (prelude:namespacep t)))
      ,(mu:cons :string (:lambda (t) (prelude:stringp t)))
      ,(mu:cons :struct (:lambda (t) (prelude:structp t)))
      ,(mu:cons :symbol (:lambda (t) (mu:eq :symbol (mu:type-of t))))
      ,(mu:cons :symbolp (:lambda (t) (prelude:symbolp t)))
      ,(mu:cons :t (:lambda (t) :t))
      ,(mu:cons :vector (:lambda (t) (prelude:vectorp t)))))

(mu:intern prelude "%type-predicate"
   (:lambda (type value)
      ((:lambda (predicate)
          (mu:apply predicate `(,value)))
       (mu:cdr (prelude:%assq type prelude:%type-predicate-map%)))))

(mu:intern prelude "%make-type"
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
                (mu:svref (mu:struct-vec (mu:symbol-value type-symbol)) 1)))
            property-values))
        (mu:find prelude:%types-ns name))
       (mu:make-struct :<type> `(,name ,property-values))))

;;;
;;; returns () if not found
;;;
(mu:intern prelude "%type-ref"
   (:lambda (prop-name struct)
      (:if (prelude:%prelude-type-p struct)
           (:if (prelude:keywordp prop-name)
                (prelude:%assq prop-name (mu:svref (mu:struct-vec struct) 1))
                (prelude:raise struct 'prelude:%type-ref "not a prelude type"))
           (prelude:raise prop-name 'prelude:%type-ref "not a property name"))))

;;;
;;; types
;;;
(mu:intern prelude "type-of"
   (:lambda (value)
      ((:lambda (type)
          (:if (mu:eq type :struct)
               (:if (mu:eq :<type> (mu:struct-type value))
                    (mu:makunbound (mu:intern (mu:find-namespace "") (mu:svref (mu:struct-vec value) 0) ()))
                    (mu:struct-type value))
               (:if (prelude:stringp value)
                    'string
                    (mu:makunbound (mu:intern (mu:find-namespace "") (mu:symbol-name type) ())))))
       (mu:type-of value))))

;;;
;;; typep
;;;
(mu:intern prelude "%typespec-map%"
   `((asyncid :asyncid)
     (bit-vector (satisfies prelude:bit-vector-p))
     (char (satisfies prelude:charp))
     (function (satisfies prelude:functionp))
     (map :map)
     (number (or (satisfies prelude:floatp) integer))
     (integer (satisfies prelude:fixnump))
     (sequence (or (satisfies prelude:vectorp) (satisfies prelude:stringp) (satisfies prelude:listp)))
     (stream (satisfies prelude:streamp))
     (string (satisfies prelude:stringp))
     (struct (satisifes prelude:structp))
     (symbol (or (satisfies prelude:symbolp) (satisfies prelude:keywordp)))))

(mu:intern prelude "%compound-typespec-map%"
   `(,(mu:cons 'satisfies
         (:lambda (value spec-list)
            (:if (mu:eq 1 (mu:length spec-list))
                 ((:lambda (predicate-symbol)
                    (:if (prelude:symbolp predicate-symbol)
                         (:if (mu:boundp predicate-symbol)
                              (prelude:apply (mu:symbol-value predicate-symbol) `(,value))
                              (prelude:raise predicate 'prelude:typep "satisfies typespec format"))
                         (prelude:raise predicate 'prelude:typep "satisfies typespec format")))
                  (mu:car spec-list))
                 (prelude:raise spec-list 'prelude:typep "satisfies typespec format"))))
     ,(mu:cons 'not (:lambda (value spec-list)
                       (:if (mu:eq 1 (mu:length spec-list))
                            (prelude:not (prelude:typep value (mu:car spec-list)))
                            (prelude:raise spec-list 'prelude:typep "no typespec format"))))
     ,(mu:cons 'and (:lambda (value spec-list)
                       (:if (prelude:null spec-list)
                            :t
                            (prelude:foldl
                             (:lambda (elt acc)
                                (prelude:%andf acc (prelude:typep value elt)))
                             :t
                             spec-list))))
     ,(mu:cons 'or (:lambda (value spec-list)
                     (:if (prelude:null spec-list)
                          ()
                          (prelude:foldl
                           (:lambda (elt acc)
                             (prelude:%orf acc (prelude:typep value elt)))
                           :t
                           spec-list))))))

(mu:intern prelude "typep"
   (:lambda (value typespec)
     (:if (prelude:keywordp typespec)
          (mu:eq typespec (mu:type-of value))
          (:if (prelude:keywordp typespec)
               (mu:eq typespec (prelude:type-of value))
               (:if (prelude:symbolp typespec)
                    ((:lambda (spec-phrase)
                       (:if (prelude:null spec-phrase)
                            (prelude:raise typespec 'prelude:typep "typespec format")
                            (prelude:typep value (mu:car (mu:cdr spec-phrase)))))
                     (prelude:%assq typespec prelude:%typespec-map%))
                    (:if (prelude:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (prelude:null spec-phrase)
                                    (prelude:raise typespec 'prelude:typep "typespec not found")
                                    (mu:apply (mu:cdr spec-phrase) `(,value ,spec-list))))
                             (prelude:%assq spec prelude:%compound-typespec-map%)))
                          (mu:car typespec)
                          (mu:cdr typespec))
                         (prelude:raise typespec 'prelude:typep "typespec format")))))))
