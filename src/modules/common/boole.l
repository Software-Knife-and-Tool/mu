;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; boolean operations
;;;
(mu:intern common "logxor"
   (:lambda (integer-1 integer-2)
     (mu:cdr
      (mu:fix
       (:lambda (loop)
         ((:lambda (index xor)
            (:if (mu:less-than index 55)
                 ((:lambda (value)
                    (mu:cons (mu:add 1 index) value))
                   (:if (mu:eq (mu:logand (mu:ash 1 index) integer-1)
                               (mu:logand (mu:ash 1 index) integer-2))
                        xor
                        (mu:logor xor (mu:ash 1 index))))
                  loop))
             (mu:car loop)
             (mu:cdr loop)))
         (mu:cons 0 0)))))

(mu:intern common "boole"
   (:lambda (op integer-1 integer-2)
     ((:lambda (op-table)
        (:if (core:keywordp op)
             (:if (core:fixnump integer-1)
                  (:if (core:fixnump integer-2)
                       ((:lambda (entry)
                          (:if (core:null entry)
                               (core:raise op 'common:boole "not an op")
                               (core:apply (mu:cdr entry) ())))
                        (core:%assq op op-table))
                       (core:raise integer-2 'common:boole "not a fixnum"))
                  (core:raise integer-1 'common:boole "not a fixnum"))
             (core:raise list 'common:boole "not a boole keyword")))
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
         ,(mu:cons :eqv (:lambda () (mu:lognot (common:logxor integer-1 integer-2))))
         ,(mu:cons :xor (:lambda () (common:logxor integer-1 integer-2)))))))
