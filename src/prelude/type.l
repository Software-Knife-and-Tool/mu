;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude types
;;;
(core:intern prelude "%types-ns" (core:make-ns "%types"))

(core:intern prelude "%prelude-type-p"
   (:lambda (value)
      (:if (prelude:structp value)
           (core:eq :<type> (core:struct-type value))
           ())))

;;;
;;; type struct: #s(:<type> #(:t name property-alist))
;;;
;;; where the property-alist is ((:predicate . :predicate) ...)
;;;
(core:intern prelude "define-type"
   (:lambda (name properties)
      ((:lambda (str)
          (:if (prelude:listp properties)
               ((:lambda (symbol)
                   (:if (prelude:null symbol)
                        (core:intern (core:find-ns "%types") str (core:make-struct :<type> `(,str ,properties)))
                        (prelude:raise symbol 'prelude:define-type "type symbol bound")))
                (core:find (core:find-ns "%types") str))
               (prelude:raise properties 'prelude:define-type "not a property list")))
       (prelude:string name))))

(core:intern prelude "%type-predicate-map%"
   `(,(core:cons :char (:lambda (t) (prelude:charp t)))
      ,(core:cons :cons (:lambda (t) (prelude:consp t)))
      ,(core:cons :fixnum (:lambda (t) (prelude:fixnump t)))
      ,(core:cons :float (:lambda (t) (prelude:floatp t)))
      ,(core:cons :func (:lambda (t) (prelude:functionp t)))
      ,(core:cons :listp (:lambda (t) (prelude:listp t)))
      ,(core:cons :map (:lambda (t) (core:eq :map (core:type-of t))))
      ,(core:cons :keyword (:lambda (t) (prelude:keywordp t)))
      ,(core:cons :ns (:lambda (t) (prelude:namespacep t)))
      ,(core:cons :string (:lambda (t) (prelude:stringp t)))
      ,(core:cons :struct (:lambda (t) (prelude:structp t)))
      ,(core:cons :symbol (:lambda (t) (core:eq :symbol (core:type-of t))))
      ,(core:cons :symbolp (:lambda (t) (prelude:symbolp t)))
      ,(core:cons :t (:lambda (t) :t))
      ,(core:cons :vector (:lambda (t) (prelude:vectorp t)))))

(core:intern prelude "%type-predicate"
   (:lambda (type value)
      ((:lambda (predicate)
          (core:apply predicate `(,value)))
       (core:cdr (prelude:%assq type prelude:%type-predicate-map%)))))

(core:intern prelude "%make-type"
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
                (core:vector-ref (core:struct-vec (core:symbol-value type-symbol)) 1)))
            property-values))
        (core:find prelude:%types-ns name))
       (core:make-struct :<type> `(,name ,property-values))))

;;;
;;; returns () if not found
;;;
(core:intern prelude "%type-ref"
   (:lambda (prop-name struct)
      (:if (prelude:%prelude-type-p struct)
           (:if (prelude:keywordp prop-name)
                (prelude:%assq prop-name (core:vector-ref (core:struct-vec struct) 1))
                (prelude:raise struct 'prelude:%type-ref "not a prelude type"))
           (prelude:raise prop-name 'prelude:%type-ref "not a property name"))))

;;;
;;; types
;;;
(core:intern prelude "type-of"
   (:lambda (value)
      ((:lambda (type)
          (:if (core:eq type :struct)
               (:if (core:eq :<type> (core:struct-type value))
                    (core:makunbound (core:intern (core:find-ns "") (core:vector-ref (core:struct-vec value) 0) ()))
                    (core:struct-type value))
               (:if (prelude:stringp value)
                    'string
                    (core:makunbound (core:intern (core:find-ns "") (core:symbol-name type) ())))))
       (core:type-of value))))

;;;
;;; typep
;;;
(core:intern prelude "%typespec-map%"
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

(core:intern prelude "%compound-typespec-map%"
   `(,(core:cons 'satisfies
         (:lambda (value spec-list)
            (:if (core:eq 1 (core:length spec-list))
                 ((:lambda (predicate-symbol)
                    (:if (prelude:symbolp predicate-symbol)
                         (:if (core:boundp predicate-symbol)
                              (prelude:apply (core:symbol-value predicate-symbol) `(,value))
                              (prelude:raise predicate 'prelude:typep "satisfies typespec format"))
                         (prelude:raise predicate 'prelude:typep "satisfies typespec format")))
                  (core:car spec-list))
                 (prelude:raise spec-list 'prelude:typep "satisfies typespec format"))))
     ,(core:cons 'not (:lambda (value spec-list)
                       (:if (core:eq 1 (core:length spec-list))
                            (prelude:not (prelude:typep value (core:car spec-list)))
                            (prelude:raise spec-list 'prelude:typep "no typespec format"))))
     ,(core:cons 'and (:lambda (value spec-list)
                       (:if (prelude:null spec-list)
                            :t
                            (prelude:foldl
                             (:lambda (elt acc)
                                (prelude:%andf acc (prelude:typep value elt)))
                             :t
                             spec-list))))
     ,(core:cons 'or (:lambda (value spec-list)
                     (:if (prelude:null spec-list)
                          ()
                          (prelude:foldl
                           (:lambda (elt acc)
                             (prelude:%orf acc (prelude:typep value elt)))
                           :t
                           spec-list))))))

(core:intern prelude "typep"
   (:lambda (value typespec)
     (:if (prelude:keywordp typespec)
          (core:eq typespec (core:type-of value))
          (:if (prelude:keywordp typespec)
               (core:eq typespec (prelude:type-of value))
               (:if (prelude:symbolp typespec)
                    ((:lambda (spec-phrase)
                       (:if (prelude:null spec-phrase)
                            (prelude:raise typespec 'prelude:typep "typespec format")
                            (prelude:typep value (core:car (core:cdr spec-phrase)))))
                     (prelude:%assq typespec prelude:%typespec-map%))
                    (:if (prelude:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (prelude:null spec-phrase)
                                    (prelude:raise typespec 'prelude:typep "typespec not found")
                                    (core:apply (core:cdr spec-phrase) `(,value ,spec-list))))
                             (prelude:%assq spec prelude:%compound-typespec-map%)))
                          (core:car typespec)
                          (core:cdr typespec))
                         (prelude:raise typespec 'prelude:typep "typespec format")))))))
