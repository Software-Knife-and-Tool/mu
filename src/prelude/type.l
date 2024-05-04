;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; types
;;;
(lib:intern prelude "type-of"
   (:lambda (value)
      ((:lambda (type)
          (:if (lib:eq type :struct)
               (:if (lib:eq :<type> (lib:st-type value))
                    (lib:untern (lib:find-ns "") (lib:sv-ref (lib:st-vec value) 0))
                    (lib:st-type value))
               (:if (prelude:stringp value)
                    'string
                    (lib:untern (lib:find-ns "") (lib:symbol-name type)))))
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
