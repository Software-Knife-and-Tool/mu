;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; types
;;;
(mu:intern :prelude "type-of"
   (:lambda (value)
      ((:lambda (type)
          (:if (mu:eq type :struct)
               (:if (mu:eq :<type> (mu:st-type value))
                    (mu:untern () (mu:sv-ref (mu:st-vec value) 0))
                    (mu:st-type value))
               (:if (prelude:stringp value)
                    'string
                    (mu:untern () (mu:sy-name type)))))
       (mu:type-of value))))

;;;
;;; typep
;;;
(mu:intern :prelude "%typespec-map%"
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

(mu:intern :prelude "%compound-typespec-map%"
   `(,(mu:cons 'satisfies
         (:lambda (value spec-list)
            (:if (mu:eq 1 (mu:length spec-list))
                 ((:lambda (predicate-symbol)
                    (:if (prelude:symbolp predicate-symbol)
                         (:if (mu:boundp predicate-symbol)
                              (prelude:apply (mu:sy-val predicate-symbol) `(,value))
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

(mu:intern :prelude "typep"
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
                     (prelude:%assoc typespec prelude:%typespec-map%))
                    (:if (prelude:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (prelude:null spec-phrase)
                                    (prelude:raise typespec 'prelude:typep "typespec not found")
                                    (mu:apply (mu:cdr spec-phrase) `(,value ,spec-list))))
                             (prelude:%assoc spec prelude:%compound-typespec-map%)))
                          (mu:car typespec)
                          (mu:cdr typespec))
                         (prelude:raise typespec 'prelude:typep "typespec format")))))))
