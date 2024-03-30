;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; boolean operations
;;;
(lib:intern :common "logxor"
   (:lambda (integer-1 integer-2)
     (lib:cdr
      (lib:fix
       (:lambda (loop)
         ((:lambda (index xor)
             (:if (lib:fx-lt index 55)
                  `(,(prelude:1+ index)
                    ,@(:if (lib:eq (lib:logand (lib:ash 1 index) integer-1)
                                  (lib:logand (lib:ash 1 index) integer-2))
                           xor
                           (lib:logor xor (lib:ash 1 index))))
                  loop))
             (lib:car loop)
             (lib:cdr loop)))
         '(0 . 0)))))

(lib:intern :common "boole"
   (:lambda (op integer-1 integer-2)
     ((:lambda (op-table)
        (:if (prelude:keywordp op)
             (:if (prelude:fixnump integer-1)
                  (:if (prelude:fixnump integer-2)
                       ((:lambda (entry)
                          (:if (prelude:null entry)
                               (prelude:raise op 'prelude:boole "not an op")
                               (lib:apply (lib:cdr entry) ())))
                        (prelude:%assoc op op-table))
                       (prelude:raise integer-2 'prelude:boole "not a fixnum"))
                  (prelude:raise integer-1 'prelude:boole "not a fixnum"))
             (prelude:raise list 'prelude:boole "not a boole keyword")))
      `(,(lib:cons :1 (:lambda () integer-1))
         ,(lib:cons :2 (:lambda () integer-2))
         ,(lib:cons :andc1 (:lambda () (lib:logand (lib:lognot integer-1) integer-2)))
         ,(lib:cons :andc2 (:lambda () (lib:logand integer-1 (lib:lognot integer-2))))
         ,(lib:cons :and (:lambda () (lib:logand integer-1 integer-2)))
         ,(lib:cons :c1 (:lambda () (lib:lognot integer-1)))
         ,(lib:cons :c2 (:lambda () (lib:lognot integer-2)))
         ,(lib:cons :clr (:lambda () 0))
         ,(lib:cons :ior (:lambda () (lib:logor integer-1 integer-2)))
         ,(lib:cons :nand (:lambda () (lib:lognot (lib:logand integer-1 integer-2))))
         ,(lib:cons :nor (:lambda () (lib:lognot (lib:logor integer-1 integer-2))))
         ,(lib:cons :orc1 (:lambda () (lib:logor (lib:lognot integer-1) integer-2)))
         ,(lib:cons :orc2 (:lambda () (lib:logor integer-1 (lib:lognot integer-2))))  
         ,(lib:cons :set (:lambda () -1))
         ,(lib:cons :eqv (:lambda () (lib:lognot (prelude:logxor integer-1 integer-2))))
         ,(lib:cons :xor (:lambda () (prelude:logxor integer-1 integer-2)))))))
