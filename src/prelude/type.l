;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; types
;;;
(core:intern :prelude "type-of"
   (:lambda (value)
      ((:lambda (type)
          (:if (core:eq type :struct)
               (:if (core:eq :<type> (core:st-type value))
                    (core:untern () (core:sv-ref (core:st-vec value) 0))
                    (core:st-type value))
               (:if (prelude:stringp value)
                    'string
                    (core:untern () (core:sy-name type)))))
       (core:type-of value))))

;;;
;;; typep
;;;
(core:intern :prelude "%typespec-map%"
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

(core:intern :prelude "%compound-typespec-map%"
   `(,(core:cons 'satisfies
         (:lambda (value spec-list)
            (:if (core:eq 1 (core:length spec-list))
                 ((:lambda (predicate-symbol)
                    (:if (prelude:symbolp predicate-symbol)
                         (:if (core:boundp predicate-symbol)
                              (prelude:apply (core:sy-val predicate-symbol) `(,value))
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

(core:intern :prelude "typep"
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
                     (prelude:%assoc typespec prelude:%typespec-map%))
                    (:if (prelude:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (prelude:null spec-phrase)
                                    (prelude:raise typespec 'prelude:typep "typespec not found")
                                    (core:apply (core:cdr spec-phrase) `(,value ,spec-list))))
                             (prelude:%assoc spec prelude:%compound-typespec-map%)))
                          (core:car typespec)
                          (core:cdr typespec))
                         (prelude:raise typespec 'prelude:typep "typespec format")))))))
