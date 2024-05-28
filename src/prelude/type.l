;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude types
;;;
(crux:intern prelude "%types-ns" (crux:make-ns "%types"))

(crux:intern prelude "%prelude-type-p"
   (:lambda (value)
      (:if (prelude:structp value)
           (crux:eq :<type> (crux:struct-type value))
           ())))

;;;
;;; type struct: #s(:<type> #(:t name property-alist))
;;;
;;; where the property-alist is ((:predicate . :predicate) ...)
;;;
(crux:intern prelude "define-type"
   (:lambda (name properties)
      ((:lambda (str)
          (:if (prelude:listp properties)
               ((:lambda (symbol)
                   (:if (prelude:null symbol)
                        (crux:intern (crux:find-ns "%types") str (crux:make-struct :<type> `(,str ,properties)))
                        (prelude:raise symbol 'prelude:define-type "type symbol bound")))
                (crux:find (crux:find-ns "%types") str))
               (prelude:raise properties 'prelude:define-type "not a property list")))
       (prelude:string name))))

(crux:intern prelude "%type-predicate-map%"
   `(,(crux:cons :char (:lambda (t) (prelude:charp t)))
      ,(crux:cons :cons (:lambda (t) (prelude:consp t)))
      ,(crux:cons :fixnum (:lambda (t) (prelude:fixnump t)))
      ,(crux:cons :float (:lambda (t) (prelude:floatp t)))
      ,(crux:cons :func (:lambda (t) (prelude:functionp t)))
      ,(crux:cons :listp (:lambda (t) (prelude:listp t)))
      ,(crux:cons :map (:lambda (t) (crux:eq :map (crux:type-of t))))
      ,(crux:cons :keyword (:lambda (t) (prelude:keywordp t)))
      ,(crux:cons :ns (:lambda (t) (prelude:namespacep t)))
      ,(crux:cons :string (:lambda (t) (prelude:stringp t)))
      ,(crux:cons :struct (:lambda (t) (prelude:structp t)))
      ,(crux:cons :symbol (:lambda (t) (crux:eq :symbol (crux:type-of t))))
      ,(crux:cons :symbolp (:lambda (t) (prelude:symbolp t)))
      ,(crux:cons :t (:lambda (t) :t))
      ,(crux:cons :vector (:lambda (t) (prelude:vectorp t)))))

(crux:intern prelude "%type-predicate"
   (:lambda (type value)
      ((:lambda (predicate)
          (crux:apply predicate `(,value)))
       (crux:cdr (prelude:%assq type prelude:%type-predicate-map%)))))

(crux:intern prelude "%make-type"
    (:lambda (name property-values)
       ((:lambda (type-symbol)
           (prelude:%mapc
            (:lambda (property-value)
               (prelude:findl
                (:lambda (property-def)
                   (:if (crux:eq (crux:car property-value) (crux:car property-def))
                        ((:lambda (type value)
                            (:if (prelude:%type-predicate type value)
                                 property-def
                                 (prelude:raise property-value '%make-type "type mismatch")))
                         (crux:cdr property-def)
                         (crux:cdr property-value))
                        ()))
                (crux:vector-ref (crux:struct-vec (crux:symbol-value type-symbol)) 1)))
            property-values))
        (crux:find prelude:%types-ns name))
       (crux:make-struct :<type> `(,name ,property-values))))

;;;
;;; returns () if not found
;;;
(crux:intern prelude "%type-ref"
   (:lambda (prop-name struct)
      (:if (prelude:%prelude-type-p struct)
           (:if (prelude:keywordp prop-name)
                (prelude:%assq prop-name (crux:vector-ref (crux:struct-vec struct) 1))
                (prelude:raise struct 'prelude:%type-ref "not a prelude type"))
           (prelude:raise prop-name 'prelude:%type-ref "not a property name"))))

;;;
;;; types
;;;
(crux:intern prelude "type-of"
   (:lambda (value)
      ((:lambda (type)
          (:if (crux:eq type :struct)
               (:if (crux:eq :<type> (crux:struct-type value))
                    (crux:makunbound (crux:intern (crux:find-ns "") (crux:vector-ref (crux:struct-vec value) 0) ()))
                    (crux:struct-type value))
               (:if (prelude:stringp value)
                    'string
                    (crux:makunbound (crux:intern (crux:find-ns "") (crux:symbol-name type) ())))))
       (crux:type-of value))))

;;;
;;; typep
;;;
(crux:intern prelude "%typespec-map%"
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

(crux:intern prelude "%compound-typespec-map%"
   `(,(crux:cons 'satisfies
         (:lambda (value spec-list)
            (:if (crux:eq 1 (crux:length spec-list))
                 ((:lambda (predicate-symbol)
                    (:if (prelude:symbolp predicate-symbol)
                         (:if (crux:boundp predicate-symbol)
                              (prelude:apply (crux:symbol-value predicate-symbol) `(,value))
                              (prelude:raise predicate 'prelude:typep "satisfies typespec format"))
                         (prelude:raise predicate 'prelude:typep "satisfies typespec format")))
                  (crux:car spec-list))
                 (prelude:raise spec-list 'prelude:typep "satisfies typespec format"))))
     ,(crux:cons 'not (:lambda (value spec-list)
                       (:if (crux:eq 1 (crux:length spec-list))
                            (prelude:not (prelude:typep value (crux:car spec-list)))
                            (prelude:raise spec-list 'prelude:typep "no typespec format"))))
     ,(crux:cons 'and (:lambda (value spec-list)
                       (:if (prelude:null spec-list)
                            :t
                            (prelude:foldl
                             (:lambda (elt acc)
                                (prelude:%andf acc (prelude:typep value elt)))
                             :t
                             spec-list))))
     ,(crux:cons 'or (:lambda (value spec-list)
                     (:if (prelude:null spec-list)
                          ()
                          (prelude:foldl
                           (:lambda (elt acc)
                             (prelude:%orf acc (prelude:typep value elt)))
                           :t
                           spec-list))))))

(crux:intern prelude "typep"
   (:lambda (value typespec)
     (:if (prelude:keywordp typespec)
          (crux:eq typespec (crux:type-of value))
          (:if (prelude:keywordp typespec)
               (crux:eq typespec (prelude:type-of value))
               (:if (prelude:symbolp typespec)
                    ((:lambda (spec-phrase)
                       (:if (prelude:null spec-phrase)
                            (prelude:raise typespec 'prelude:typep "typespec format")
                            (prelude:typep value (crux:car (crux:cdr spec-phrase)))))
                     (prelude:%assq typespec prelude:%typespec-map%))
                    (:if (prelude:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (prelude:null spec-phrase)
                                    (prelude:raise typespec 'prelude:typep "typespec not found")
                                    (crux:apply (crux:cdr spec-phrase) `(,value ,spec-list))))
                             (prelude:%assq spec prelude:%compound-typespec-map%)))
                          (crux:car typespec)
                          (crux:cdr typespec))
                         (prelude:raise typespec 'prelude:typep "typespec format")))))))
