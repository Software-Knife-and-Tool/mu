;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; boolean operations
;;;
(core:intern :common "logxor"
   (:lambda (integer-1 integer-2)
     (core:cdr
      (core:fix
       (:lambda (loop)
         ((:lambda (index xor)
             (:if (core:fx-lt index 55)
                  `(,(prelude:1+ index)
                    ,@(:if (core:eq (core:logand (core:ash 1 index) integer-1)
                                  (core:logand (core:ash 1 index) integer-2))
                           xor
                           (core:logor xor (core:ash 1 index))))
                  loop))
             (core:car loop)
             (core:cdr loop)))
         '(0 . 0)))))

(core:intern :common "boole"
   (:lambda (op integer-1 integer-2)
     ((:lambda (op-table)
        (:if (prelude:keywordp op)
             (:if (prelude:fixnump integer-1)
                  (:if (prelude:fixnump integer-2)
                       ((:lambda (entry)
                          (:if (prelude:null entry)
                               (prelude:raise op 'prelude:boole "not an op")
                               (core:apply (core:cdr entry) ())))
                        (prelude:%assoc op op-table))
                       (prelude:raise integer-2 'prelude:boole "not a fixnum"))
                  (prelude:raise integer-1 'prelude:boole "not a fixnum"))
             (prelude:raise list 'prelude:boole "not a boole keyword")))
      `(,(core:cons :1 (:lambda () integer-1))
         ,(core:cons :2 (:lambda () integer-2))
         ,(core:cons :andc1 (:lambda () (core:logand (core:lognot integer-1) integer-2)))
         ,(core:cons :andc2 (:lambda () (core:logand integer-1 (core:lognot integer-2))))
         ,(core:cons :and (:lambda () (core:logand integer-1 integer-2)))
         ,(core:cons :c1 (:lambda () (core:lognot integer-1)))
         ,(core:cons :c2 (:lambda () (core:lognot integer-2)))
         ,(core:cons :clr (:lambda () 0))
         ,(core:cons :ior (:lambda () (core:logor integer-1 integer-2)))
         ,(core:cons :nand (:lambda () (core:lognot (core:logand integer-1 integer-2))))
         ,(core:cons :nor (:lambda () (core:lognot (core:logor integer-1 integer-2))))
         ,(core:cons :orc1 (:lambda () (core:logor (core:lognot integer-1) integer-2)))
         ,(core:cons :orc2 (:lambda () (core:logor integer-1 (core:lognot integer-2))))  
         ,(core:cons :set (:lambda () -1))
         ,(core:cons :eqv (:lambda () (core:lognot (prelude:logxor integer-1 integer-2))))
         ,(core:cons :xor (:lambda () (prelude:logxor integer-1 integer-2)))))))
