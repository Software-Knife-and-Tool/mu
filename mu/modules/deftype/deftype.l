;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; deftype
;;;
(mu:intern deftype "%types/" (mu:make-namespace "deftype/types"))

;;;
;;; type struct: #s(:<type> #(:t name property-alist))
;;;
;;; where the property-alist is ((:property . accessor) ...)
;;;
(mu:intern deftype "%deftype"
   (:lambda (name properties)
     (:if (core:listp properties)
          ((:lambda (symbol)
             (:if (core:null symbol)
                  (mu:intern deftype:%types/ name (mu:make-struct :<type> (core:%list2 name properties)))
                  (mu:raise symbol 'deftype:%deftype :unbound)))
           (mu:find deftype:%types/ name))
          (mu:raise properties 'deftype:%deftype :type))))

;;;
;;; deftype deftype
;;;

;;;
;;;  (:name   . :symbolp)    name symbol
;;;  (:spec   . :t)          typespec form
;;;
(deftype:%deftype "%deftype"
    '((:name . :symbolp)
      (:spec . :t)))

(mu:intern deftype "%deftype-prop"
   (:lambda (prop deftype)
     (mu:cdr (deftype:%type-prop deftype))))

(mu:intern deftype "%make-deftype"
   (:lambda (name spec)
     (deftype:%make-type "%deftype"
        `(,(mu:cons :name name)
          ,(mu:cons :spec spec)))))

(mu:intern deftype "%deftypep"
   (:lambda (type)
     (:if (core:%typep type)
          (mu:eq '%deftype (core:type-of type))
          ())))

;;;
;;; typep
;;;
(mu:intern deftype "%typespec-map%"
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

(mu:intern deftype "%compound-typespec-map%"
   `(,(mu:cons 'satisfies
         (:lambda (value spec-list)
            (:if (mu:eq 1 (mu:length spec-list))
                 ((:lambda (predicate-symbol)
                    (:if (core:symbolp predicate-symbol)
                         (:if (mu:boundp predicate-symbol)
                              (core:apply (mu:symbol-value predicate-symbol) (mu:cons value ()))
                              (mu:raise predicate-symbol 'deftype:typep :range))
                         (mu:raise predicate-symbol 'deftype:typep :range)))
                  (mu:car spec-list))
                 (mu:raise spec-list 'deftype:typep :syntax))))
     ,(mu:cons 'not (:lambda (value spec-list)
                       (:if (mu:eq 1 (mu:length spec-list))
                            (core:null (deftype:typep value (mu:car spec-list)))
                            (mu:raise spec-list 'deftype:typep :syntax))))
     ,(mu:cons 'and (:lambda (value spec-list)
                       (:if (core:null spec-list)
                            :t
                            (core:foldl
                             (:lambda (elt acc)
                                (core:%and acc (deftype:typep value elt)))
                             :t
                             spec-list))))
     ,(mu:cons 'or (:lambda (value spec-list)
                     (:if (core:null spec-list)
                          ()
                          (core:foldl
                           (:lambda (elt acc)
                             (core:%or acc (deftype:typep value elt)))
                           :t
                           spec-list))))))

(mu:intern deftype "%typep"
   (:lambda (value typespec)
     (:if (core:keywordp typespec)
          (mu:eq typespec (mu:type-of value))
          (:if (deftype:%deftypep typespec)
               (deftype:typep value (deftype:%deftype-prop :spec typespec))
               (:if (core:symbolp typespec)
                    ((:lambda (spec-phrase)
                       (:if (core:null spec-phrase)
                            (mu:raise typespec 'deftype:typep :syntax)
                            (deftype:typep value (mu:car (mu:cdr spec-phrase)))))
                     (core:assq typespec deftype:%typespec-map%))
                    (:if (core:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (core:null spec-phrase)
                                    (mu:raise typespec 'deftype:typep :syntax)
                                    (mu:apply (mu:cdr spec-phrase) (core:%list2 value spec-list))))
                             (core:assq spec deftype:%compound-typespec-map%)))
                          (mu:car typespec)
                          (mu:cdr typespec))
                         (mu:raise typespec 'deftype:typep :synatx)))))))   

;;;
;;; types
;;;
(mu:intern deftype "typep"
   (:lambda (value typespec)
     (:if (core:keywordp typespec)
          (mu:eq typespec (mu:type-of value))
          (:if (deftype:%deftypep typespec)
               (deftype:typep value (deftype:%deftype-prop :spec typespec))
               (:if (core:symbolp typespec)
                    ((:lambda (spec-phrase)
                       (:if (core:null spec-phrase)
                            (mu:raise typespec 'deftype:typep :syntax)
                            (deftype:typep value (mu:car (mu:cdr spec-phrase)))))
                     (core:assq typespec deftype:%typespec-map%))
                    (:if (core:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (core:null spec-phrase)
                                    (mu:raise typespec 'deftype:typep :range)
                                    (mu:apply (mu:cdr spec-phrase) (core:%list2 value spec-list))))
                             (core:assq spec deftype:%compound-typespec-map%)))
                          (mu:car typespec)
                          (mu:cdr typespec))
                         (mu:raise typespec 'deftype:typep :syntax)))))))
