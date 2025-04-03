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

(mu:intern common "%deftype-prop"
   (:lambda (prop deftype)
     (mu:cdr (core:%type-ref deftype))))

(mu:intern common "%make-deftype"
   (:lambda (name spec)
     (core:%make-core-type "%deftype"
        `(,(mu:cons :name name)
          ,(mu:cons :spec spec)))))

(mu:intern common "%deftypep"
   (:lambda (type)
     (:if (core:%core-type-p type)
          (mu:eq '%deftype (core:type-of type))
          ())))

(defmacro deftype (name lambda &rest body)
  (mu:intern
   (mu:symbol-namespace)
   (mu:symbol-name)
   (common:%make-deftype name `(mu:apply (:lambda () ,@body) ())))
  `(:quote ,@name))

;;;
;;; typep
;;;
(mu:intern common "%typespec-map%"
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

(mu:intern common "%compound-typespec-map%"
   `(,(mu:cons 'satisfies
         (:lambda (value spec-list)
            (:if (mu:eq 1 (mu:length spec-list))
                 ((:lambda (predicate-symbol)
                    (:if (core:symbolp predicate-symbol)
                         (:if (mu:boundp predicate-symbol)
                              (core:apply (mu:symbol-value predicate-symbol) (mu:cons value ()))
                              (core:raise predicate 'common:typep "satisfies typespec symbol"))
                         (core:raise predicate 'common:typep "satisfies typespec symbol")))
                  (mu:car spec-list))
                 (core:raise spec-list 'common:typep "satisfies typespec format"))))
     ,(mu:cons 'not (:lambda (value spec-list)
                       (:if (mu:eq 1 (mu:length spec-list))
                            (core:%not (common:typep value (mu:car spec-list)))
                            (core:raise spec-list 'common:typep "missing typespec format"))))
     ,(mu:cons 'and (:lambda (value spec-list)
                       (:if (core:null spec-list)
                            :t
                            (core:%foldl
                             (:lambda (elt acc)
                                (core:%and acc (common:typep value elt)))
                             :t
                             spec-list))))
     ,(mu:cons 'or (:lambda (value spec-list)
                     (:if (core:null spec-list)
                          ()
                          (core:%foldl
                           (:lambda (elt acc)
                             (core:%or acc (common:typep value elt)))
                           :t
                           spec-list))))))

(mu:intern common "typep"
   (:lambda (value typespec)
     (:if (core:keywordp typespec)
          (mu:eq typespec (mu:type-of value))
          (:if (common:%deftypep typespec)
               (common:typep value (common:%deftype-prop :spec typespec))
               (:if (core:symbolp typespec)
                    ((:lambda (spec-phrase)
                       (:if (core:null spec-phrase)
                            (core:raise typespec 'common:typep "typespec format")
                            (common:typep value (mu:car (mu:cdr spec-phrase)))))
                     (core:%assq typespec common:%typespec-map%))
                    (:if (core:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (core:null spec-phrase)
                                    (core:raise typespec 'common:typep "typespec not found")
                                    (mu:apply (mu:cdr spec-phrase) (core:%list2 value spec-list))))
                             (core:%assq spec common:%compound-typespec-map%)))
                          (mu:car typespec)
                          (mu:cdr typespec))
                         (core:raise typespec 'common:typep "typespec format")))))))
   
