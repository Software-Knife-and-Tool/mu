;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

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
                              (core:raise predicate 'core:typep "satisfies typespec symbol"))
                         (core:raise predicate 'core:typep "satisfies typespec symbol")))
                  (mu:car spec-list))
                 (core:raise spec-list 'core:typep "satisfies typespec format"))))
     ,(mu:cons 'not (:lambda (value spec-list)
                       (:if (mu:eq 1 (mu:length spec-list))
                            (core:%not (core:typep value (mu:car spec-list)))
                            (core:raise spec-list 'core:typep "missing typespec format"))))
     ,(mu:cons 'and (:lambda (value spec-list)
                       (:if (core:null spec-list)
                            :t
                            (core:%foldl
                             (:lambda (elt acc)
                                (core:%and acc (core:typep value elt)))
                             :t
                             spec-list))))
     ,(mu:cons 'or (:lambda (value spec-list)
                     (:if (core:null spec-list)
                          ()
                          (core:%foldl
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
                            (core:raise typespec 'core:typep "typespec format")
                            (core:typep value (mu:car (mu:cdr spec-phrase)))))
                     (core:%assq typespec core:%typespec-map%))
                    (:if (core:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (core:null spec-phrase)
                                    (core:raise typespec 'core:typep "typespec not found")
                                    (mu:apply (mu:cdr spec-phrase) (core:%list2 value spec-list))))
                             (core:%assq spec core:%compound-typespec-map%)))
                          (mu:car typespec)
                          (mu:cdr typespec))
                         (core:raise typespec 'core:typep "typespec format")))))))   
