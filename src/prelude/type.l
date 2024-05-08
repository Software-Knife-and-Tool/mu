;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude types
;;;
(lib:intern prelude "%types-ns" (lib:make-ns "%types"))

(lib:intern prelude "%prelude-type-p"
   (:lambda (value)
      (:if (prelude:structp value)
           (lib:eq :<type> (lib:struct-type value))
           ())))

;;;
;;; type struct: #s(:<type> #(:t name property-alist))
;;;
;;; where the property-alist is ((:predicate . :predicate) ...)
;;;
(lib:intern prelude "define-type"
   (:lambda (name properties)
      ((:lambda (str)
          (:if (prelude:listp properties)
               ((:lambda (symbol)
                   (:if (prelude:null symbol)
                        (lib:intern (lib:find-ns "%types") str (lib:make-struct :<type> `(,str ,properties)))
                        (prelude:raise symbol 'prelude:define-type "type symbol bound")))
                (lib:find (lib:find-ns "%types") str))
               (prelude:raise properties 'prelude:define-type "not a property list")))
       (prelude:string name))))

(lib:intern prelude "%type-predicate-map%"
   `(,(lib:cons :char (:lambda (t) (prelude:charp t)))
      ,(lib:cons :cons (:lambda (t) (prelude:consp t)))
      ,(lib:cons :fixnum (:lambda (t) (prelude:fixnump t)))
      ,(lib:cons :float (:lambda (t) (prelude:floatp t)))
      ,(lib:cons :func (:lambda (t) (prelude:functionp t)))
      ,(lib:cons :listp (:lambda (t) (prelude:listp t)))
      ,(lib:cons :map (:lambda (t) (lib:eq :map (lib:type-of t))))
      ,(lib:cons :keyword (:lambda (t) (prelude:keywordp t)))
      ,(lib:cons :ns (:lambda (t) (prelude:namespacep t)))
      ,(lib:cons :string (:lambda (t) (prelude:stringp t)))
      ,(lib:cons :struct (:lambda (t) (prelude:structp t)))
      ,(lib:cons :symbol (:lambda (t) (lib:eq :symbol (lib:type-of t))))
      ,(lib:cons :symbolp (:lambda (t) (prelude:symbolp t)))
      ,(lib:cons :t (:lambda (t) :t))
      ,(lib:cons :vector (:lambda (t) (prelude:vectorp t)))))

(lib:intern prelude "%type-predicate"
   (:lambda (type value)
      ((:lambda (predicate)
          (lib:apply predicate `(,value)))
       (lib:cdr (prelude:%assoc type prelude:%type-predicate-map%)))))

(lib:intern prelude "%make-type"
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
                (lib:vector-ref (lib:struct-vec (lib:symbol-value type-symbol)) 1)))
            property-values))
        (lib:find prelude:%types-ns name))
       (lib:make-struct :<type> `(,name ,property-values))))

;;;
;;; returns () if not found
;;;
(lib:intern prelude "%type-ref"
   (:lambda (prop-name struct)
      (:if (prelude:%prelude-type-p struct)
           (:if (prelude:keywordp prop-name)
                (prelude:%assoc prop-name (lib:vector-ref (lib:struct-vec struct) 1))
                (prelude:raise struct 'prelude:%type-ref "not a prelude type"))
           (prelude:raise prop-name 'prelude:%type-ref "not a property name"))))

;;;
;;; types
;;;
(lib:intern prelude "type-of"
   (:lambda (value)
      ((:lambda (type)
          (:if (lib:eq type :struct)
               (:if (lib:eq :<type> (lib:struct-type value))
                    (lib:makunbound (lib:intern (lib:find-ns "") (lib:vector-ref (lib:struct-vec value) 0) ()))
                    (lib:struct-type value))
               (:if (prelude:stringp value)
                    'string
                    (lib:makunbound (lib:intern (lib:find-ns "") (lib:symbol-name type) ())))))
       (lib:type-of value))))

;;;
;;; typep
;;;
(lib:intern prelude "%typespec-map%"
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

(lib:intern prelude "%compound-typespec-map%"
   `(,(lib:cons 'satisfies
         (:lambda (value spec-list)
            (:if (lib:eq 1 (lib:length spec-list))
                 ((:lambda (predicate-symbol)
                    (:if (prelude:symbolp predicate-symbol)
                         (:if (lib:boundp predicate-symbol)
                              (prelude:apply (lib:symbol-value predicate-symbol) `(,value))
                              (prelude:raise predicate 'prelude:typep "satisfies typespec format"))
                         (prelude:raise predicate 'prelude:typep "satisfies typespec format")))
                  (lib:car spec-list))
                 (prelude:raise spec-list 'prelude:typep "satisfies typespec format"))))
     ,(lib:cons 'not (:lambda (value spec-list)
                       (:if (lib:eq 1 (lib:length spec-list))
                            (prelude:not (prelude:typep value (lib:car spec-list)))
                            (prelude:raise spec-list 'prelude:typep "no typespec format"))))
     ,(lib:cons 'and (:lambda (value spec-list)
                       (:if (prelude:null spec-list)
                            :t
                            (prelude:foldl
                             (:lambda (elt acc)
                                (prelude:%andf acc (prelude:typep value elt)))
                             :t
                             spec-list))))
     ,(lib:cons 'or (:lambda (value spec-list)
                     (:if (prelude:null spec-list)
                          ()
                          (prelude:foldl
                           (:lambda (elt acc)
                             (prelude:%orf acc (prelude:typep value elt)))
                           :t
                           spec-list))))))

(lib:intern prelude "typep"
   (:lambda (value typespec)
     (:if (prelude:keywordp typespec)
          (lib:eq typespec (lib:type-of value))
          (:if (prelude:keywordp typespec)
               (lib:eq typespec (prelude:type-of value))
               (:if (prelude:symbolp typespec)
                    ((:lambda (spec-phrase)
                       (:if (prelude:null spec-phrase)
                            (prelude:raise typespec 'prelude:typep "typespec format")
                            (prelude:typep value (lib:car (lib:cdr spec-phrase)))))
                     (prelude:%assoc typespec prelude:%typespec-map%))
                    (:if (prelude:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (prelude:null spec-phrase)
                                    (prelude:raise typespec 'prelude:typep "typespec not found")
                                    (lib:apply (lib:cdr spec-phrase) `(,value ,spec-list))))
                             (prelude:%assoc spec prelude:%compound-typespec-map%)))
                          (lib:car typespec)
                          (lib:cdr typespec))
                         (prelude:raise typespec 'prelude:typep "typespec format")))))))
