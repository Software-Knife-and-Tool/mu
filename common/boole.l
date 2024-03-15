;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; boolean operations
;;;
(mu:intern :common "logxor"
   (:lambda (integer-1 integer-2)
     (mu:cdr
      (mu:fix
       (:lambda (loop)
         ((:lambda (index xor)
             (:if (mu:fx-lt index 55)
                  `(,(prelude:1+ index)
                    ,@(:if (mu:eq (mu:logand (mu:ash 1 index) integer-1)
                                  (mu:logand (mu:ash 1 index) integer-2))
                           xor
                           (mu:logor xor (mu:ash 1 index))))
                  loop))
             (mu:car loop)
             (mu:cdr loop)))
         '(0 . 0)))))

(mu:intern :common "boole"
   (:lambda (op integer-1 integer-2)
     ((:lambda (op-table)
        (:if (prelude:keywordp op)
             (:if (prelude:fixnump integer-1)
                  (:if (prelude:fixnump integer-2)
                       ((:lambda (entry)
                          (:if (prelude:null entry)
                               (prelude:raise op 'prelude:boole "not an op")
                               (mu:apply (mu:cdr entry) ())))
                        (prelude:%assoc op op-table))
                       (prelude:raise integer-2 'prelude:boole "not a fixnum"))
                  (prelude:raise integer-1 'prelude:boole "not a fixnum"))
             (prelude:raise list 'prelude:boole "not a boole keyword")))
      `(,(mu:cons :1 (:lambda () integer-1))
         ,(mu:cons :2 (:lambda () integer-2))
         ,(mu:cons :andc1 (:lambda () (mu:logand (mu:lognot integer-1) integer-2)))
         ,(mu:cons :andc2 (:lambda () (mu:logand integer-1 (mu:lognot integer-2))))
         ,(mu:cons :and (:lambda () (mu:logand integer-1 integer-2)))
         ,(mu:cons :c1 (:lambda () (mu:lognot integer-1)))
         ,(mu:cons :c2 (:lambda () (mu:lognot integer-2)))
         ,(mu:cons :clr (:lambda () 0))
         ,(mu:cons :ior (:lambda () (mu:logor integer-1 integer-2)))
         ,(mu:cons :nand (:lambda () (mu:lognot (mu:logand integer-1 integer-2))))
         ,(mu:cons :nor (:lambda () (mu:lognot (mu:logor integer-1 integer-2))))
         ,(mu:cons :orc1 (:lambda () (mu:logor (mu:lognot integer-1) integer-2)))
         ,(mu:cons :orc2 (:lambda () (mu:logor integer-1 (mu:lognot integer-2))))  
         ,(mu:cons :set (:lambda () -1))
         ,(mu:cons :eqv (:lambda () (mu:lognot (prelude:logxor integer-1 integer-2))))
         ,(mu:cons :xor (:lambda () (prelude:logxor integer-1 integer-2)))))))
