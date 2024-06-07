;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; vectors
;;;
(mu:intern prelude "vectorp"
   (:lambda (vector)
     (:if (mu:eq :vector (mu:type-of vector))
          :t
          (mu:eq '%vector (prelude:type-of vector)))))

(mu:intern prelude "bit-vector-p"
   (:lambda (vector)
     (:if (prelude:%prelude-type-p vector)
          (:if (mu:eq '%vector (prelude:type-of vector))
               (prelude:not (prelude:null (prelude:%vector-prop :bitvec vector)))
               ())
          ())))

(mu:intern prelude "vector-displaced-p"
   (:lambda (vector)
     (:if (prelude:%prelude-type-p vector)
          (:if (mu:eq '%vector (prelude:type-of vector))
               (prelude:%vector-prop :disp vector)
               ())
          ())))

(mu:intern prelude "make-vector"
   (:lambda (list)
     (:if (prelude:listp list)
          (mu:make-vector (prelude:%specialized-vector-type list) list)
          (prelude:raise list 'prelude:make-vector "not a list"))))

(mu:intern prelude "slice"
   (:lambda (vector start length)
     (:if (prelude:typep vector :vector)
          (:if (prelude:fixnump start)
               (:if (prelude:%orf (prelude:minusp start) (mu:less-than (prelude:vector-length vector) start))
                    (prelude:raise start 'prelude:slice "illegal start")
                    (:if (prelude:fixnump length)
                         (:if (prelude:%orf (prelude:minusp length) (mu:less-than (prelude:vector-length vector) length))
                              (prelude:raise length 'prelude:slice "illegal length")
                              (prelude:%make-vector vector `(,start ,@length) ()))
                         (prelude:raise start 'prelude:slice "not a fixnum")))
               (prelude:raise length 'prelude:slice "not a fixnum"))
          (prelude:raise vector 'prelude:slice "not a base vector"))))

(mu:intern prelude "vector-length"
   (:lambda (vector)
     (:if (prelude:vectorp vector)
          (:if (prelude:bit-vector-p vector)
               (mu:car (prelude:%vector-prop :bitvec vector))
               (:if (prelude:typep vector :vector)
                    (mu:vector-len vector)
                    (mu:cdr (prelude:%vector-prop :disp vector))))
          (prelude:raise vector 'prelude:vector-length "not a vector"))))

(mu:intern prelude "vector-type"
   (:lambda (vector)
     (:if (prelude:vectorp vector)
          (:if (prelude:bit-vector-p vector)
               'bit
               (:if (prelude:typep vector :vector)
                    (mu:vector-type vector)
                    (mu:vector-type (prelude:%vector-prop :base vector))))
          (prelude:raise vector 'prelude:vector-type "not a vector"))))

(mu:intern prelude "vector-ref"
   (:lambda (vector nth)
     (:if (prelude:vectorp vector)
          (:if (prelude:fixnump nth)
               (:if (prelude:%orf (prelude:minusp nth) (mu:less-than (prelude:vector-length vector) nth))
                    (prelude:raise length 'prelude:vector-ref "illegal index")
                    (:if (prelude:bit-vector-p vector)
                         ((:lambda (nth-byte offset)
                            ((:lambda (byte)
                               (:if (prelude:zerop (mu:logand byte (mu:ash 1 (mu:difference 7 offset))))
                                    0
                                    1))
                               (prelude:vector-ref (prelude:%vector-prop :base vector) nth-byte)))
                          (mu:quotient nth 8)
                          (mu:difference nth (mu:product 8 (mu:quotient nth 8))))
                         (:if (prelude:typep vector :vector)
                              (mu:vector-ref vector nth)
                              ((:lambda (base start)
                                 (mu:vector-ref base (mu:sum start nth)))
                               (prelude:%vector-prop :base vector)
                               (mu:car (prelude:%vector-prop :disp vector))))))
                    (prelude:raise nth 'prelude:vector-ref "not a fixnum"))
          (prelude:raise prop 'prelude:vector-ref "not a vector"))))

;;;
;;; write
;;;
(mu:intern prelude "%vector-write"
   (:lambda (vector escape stream)
       (:if (prelude:typep vector :vector)
           (mu:write vector escape stream)
           ((:lambda (length)
               (prelude:format stream "#(" ())
               (mu:fix
                (:lambda (index)
                   (:if (mu:less-than index length)
                        ((:lambda ()
                            (prelude:write (prelude:vector-ref vector index) escape stream)
                            (:if (mu:less-than index (prelude:1- length))
                                 (prelude:format stream " " ())
                                 ())
                            (prelude:1+ index)))
                        index))
                0)
               (prelude:format stream ")" ()))
            (prelude:vector-length vector)))
      ()))

;;;
;;; implementations
;;;
(mu:intern prelude "%make-vector"
   (:lambda (vector displace bitvec)
     (prelude:%make-type "%vector"
       `(,(mu:cons :base   vector)
         ,(mu:cons :disp   displace)
         ,(mu:cons :bitvec bitvec)))))

(mu:intern prelude "%specialized-vector-type"
   (:lambda (list)
     ((:lambda (type)
        (:if (prelude:findl (:lambda (stype) (mu:eq type stype)) '(:t :char :fixnum :float))
             (:if (mu:eq type :fixnum)
                  ((:lambda (is-byte)
                     (:if is-byte
                          :byte
                          :fixnum))
                   (prelude:foldl
                    (:lambda (elt acc)
                      (:if (prelude:%andf (prelude:not (prelude:minusp elt)) (mu:less-than elt 256))
                           acc
                           ()))
                      :t
                      list))
                  type)
             :t))
      (prelude:foldl
       (:lambda (elt acc)
          (:if (mu:eq :t acc)
               :t
               (:if (mu:eq acc (mu:type-of elt))
                    acc
                    ())))
      (mu:type-of (mu:car list))
      (mu:cdr list)))))

(mu:intern prelude "%vector-prop"
   (:lambda (prop vector)
      (:if (prelude:findl (:lambda (vec-prop) (mu:eq prop vec-prop)) '(:base :disp :bitvec))
           (mu:cdr (prelude:%type-ref prop vector))
           (prelude:raise prop 'prelude:%vector-prop "not a vector property"))))

(mu:intern prelude "%map-vector"
  (:lambda (fn vector)
    (mu:fix
     (:lambda (nth)
       (:if nth
            (:if (mu:eq nth (mu:vector-len vector))
                 ()
                 ((:lambda ()
                     (prelude:apply fn `(,(mu:vector-ref vector nth)))
                     (mu:sum 1 nth))))
            ()))
     0)))
