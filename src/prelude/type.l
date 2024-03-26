;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; types
;;;
(libcore:intern :prelude "type-of"
   (:lambda (value)
      ((:lambda (type)
          (:if (libcore:eq type :struct)
               (:if (libcore:eq :<type> (libcore:st-type value))
                    (libcore:untern () (libcore:sv-ref (libcore:st-vec value) 0))
                    (libcore:st-type value))
               (:if (prelude:stringp value)
                    'string
                    (libcore:untern () (libcore:sy-name type)))))
       (libcore:type-of value))))

;;;
;;; typep
;;;
(libcore:intern :prelude "%typespec-map%"
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

(libcore:intern :prelude "%compound-typespec-map%"
   `(,(libcore:cons 'satisfies
         (:lambda (value spec-list)
            (:if (libcore:eq 1 (libcore:length spec-list))
                 ((:lambda (predicate-symbol)
                    (:if (prelude:symbolp predicate-symbol)
                         (:if (libcore:boundp predicate-symbol)
                              (prelude:apply (libcore:sy-val predicate-symbol) `(,value))
                              (prelude:raise predicate 'prelude:typep "satisfies typespec format"))
                         (prelude:raise predicate 'prelude:typep "satisfies typespec format")))
                  (libcore:car spec-list))
                 (prelude:raise spec-list 'prelude:typep "satisfies typespec format"))))
     ,(libcore:cons 'not (:lambda (value spec-list)
                       (:if (libcore:eq 1 (libcore:length spec-list))
                            (prelude:not (prelude:typep value (libcore:car spec-list)))
                            (prelude:raise spec-list 'prelude:typep "no typespec format"))))
     ,(libcore:cons 'and (:lambda (value spec-list)
                       (:if (prelude:null spec-list)
                            :t
                            (prelude:foldl
                             (:lambda (elt acc)
                                (prelude:%andf acc (prelude:typep value elt)))
                             :t
                             spec-list))))
     ,(libcore:cons 'or (:lambda (value spec-list)
                     (:if (prelude:null spec-list)
                          ()
                          (prelude:foldl
                           (:lambda (elt acc)
                             (prelude:%orf acc (prelude:typep value elt)))
                           :t
                           spec-list))))))

(libcore:intern :prelude "typep"
   (:lambda (value typespec)
     (:if (prelude:keywordp typespec)
          (libcore:eq typespec (libcore:type-of value))
          (:if (prelude:keywordp typespec)
               (libcore:eq typespec (prelude:type-of value))
               (:if (prelude:symbolp typespec)
                    ((:lambda (spec-phrase)
                       (:if (prelude:null spec-phrase)
                            (prelude:raise typespec 'prelude:typep "typespec format")
                            (prelude:typep value (libcore:car (libcore:cdr spec-phrase)))))
                     (prelude:%assoc typespec prelude:%typespec-map%))
                    (:if (prelude:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (prelude:null spec-phrase)
                                    (prelude:raise typespec 'prelude:typep "typespec not found")
                                    (libcore:apply (libcore:cdr spec-phrase) `(,value ,spec-list))))
                             (prelude:%assoc spec prelude:%compound-typespec-map%)))
                          (libcore:car typespec)
                          (libcore:cdr typespec))
                         (prelude:raise typespec 'prelude:typep "typespec format")))))))
