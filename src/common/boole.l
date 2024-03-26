;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; boolean operations
;;;
(libcore:intern :common "logxor"
   (:lambda (integer-1 integer-2)
     (libcore:cdr
      (libcore:fix
       (:lambda (loop)
         ((:lambda (index xor)
             (:if (libcore:fx-lt index 55)
                  `(,(prelude:1+ index)
                    ,@(:if (libcore:eq (libcore:logand (libcore:ash 1 index) integer-1)
                                  (libcore:logand (libcore:ash 1 index) integer-2))
                           xor
                           (libcore:logor xor (libcore:ash 1 index))))
                  loop))
             (libcore:car loop)
             (libcore:cdr loop)))
         '(0 . 0)))))

(libcore:intern :common "boole"
   (:lambda (op integer-1 integer-2)
     ((:lambda (op-table)
        (:if (prelude:keywordp op)
             (:if (prelude:fixnump integer-1)
                  (:if (prelude:fixnump integer-2)
                       ((:lambda (entry)
                          (:if (prelude:null entry)
                               (prelude:raise op 'prelude:boole "not an op")
                               (libcore:apply (libcore:cdr entry) ())))
                        (prelude:%assoc op op-table))
                       (prelude:raise integer-2 'prelude:boole "not a fixnum"))
                  (prelude:raise integer-1 'prelude:boole "not a fixnum"))
             (prelude:raise list 'prelude:boole "not a boole keyword")))
      `(,(libcore:cons :1 (:lambda () integer-1))
         ,(libcore:cons :2 (:lambda () integer-2))
         ,(libcore:cons :andc1 (:lambda () (libcore:logand (libcore:lognot integer-1) integer-2)))
         ,(libcore:cons :andc2 (:lambda () (libcore:logand integer-1 (libcore:lognot integer-2))))
         ,(libcore:cons :and (:lambda () (libcore:logand integer-1 integer-2)))
         ,(libcore:cons :c1 (:lambda () (libcore:lognot integer-1)))
         ,(libcore:cons :c2 (:lambda () (libcore:lognot integer-2)))
         ,(libcore:cons :clr (:lambda () 0))
         ,(libcore:cons :ior (:lambda () (libcore:logor integer-1 integer-2)))
         ,(libcore:cons :nand (:lambda () (libcore:lognot (libcore:logand integer-1 integer-2))))
         ,(libcore:cons :nor (:lambda () (libcore:lognot (libcore:logor integer-1 integer-2))))
         ,(libcore:cons :orc1 (:lambda () (libcore:logor (libcore:lognot integer-1) integer-2)))
         ,(libcore:cons :orc2 (:lambda () (libcore:logor integer-1 (libcore:lognot integer-2))))  
         ,(libcore:cons :set (:lambda () -1))
         ,(libcore:cons :eqv (:lambda () (libcore:lognot (prelude:logxor integer-1 integer-2))))
         ,(libcore:cons :xor (:lambda () (prelude:logxor integer-1 integer-2)))))))
