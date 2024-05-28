;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; vectors
;;;
(crux:intern prelude "vectorp"
   (:lambda (vector)
     (:if (crux:eq :vector (crux:type-of vector))
          :t
          (crux:eq '%vector (prelude:type-of vector)))))

(crux:intern prelude "bit-vector-p"
   (:lambda (vector)
     (:if (prelude:%prelude-type-p vector)
          (:if (crux:eq '%vector (prelude:type-of vector))
               (prelude:not (prelude:null (prelude:%vector-prop :bitvec vector)))
               ())
          ())))

(crux:intern prelude "vector-displaced-p"
   (:lambda (vector)
     (:if (prelude:%prelude-type-p vector)
          (:if (crux:eq '%vector (prelude:type-of vector))
               (prelude:%vector-prop :disp vector)
               ())
          ())))

(crux:intern prelude "make-vector"
   (:lambda (list)
     (:if (prelude:listp list)
          (crux:make-vector (prelude:%specialized-vector-type list) list)
          (prelude:raise list 'prelude:make-vector "not a list"))))

(crux:intern prelude "slice"
   (:lambda (vector start length)
     (:if (prelude:typep vector :vector)
          (:if (prelude:fixnump start)
               (:if (prelude:%orf (prelude:minusp start) (crux:less-than (prelude:vector-length vector) start))
                    (prelude:raise start 'prelude:slice "illegal start")
                    (:if (prelude:fixnump length)
                         (:if (prelude:%orf (prelude:minusp length) (crux:less-than (prelude:vector-length vector) length))
                              (prelude:raise length 'prelude:slice "illegal length")
                              (prelude:%make-vector vector `(,start ,@length) ()))
                         (prelude:raise start 'prelude:slice "not a fixnum")))
               (prelude:raise length 'prelude:slice "not a fixnum"))
          (prelude:raise vector 'prelude:slice "not a base vector"))))

(crux:intern prelude "vector-length"
   (:lambda (vector)
     (:if (prelude:vectorp vector)
          (:if (prelude:bit-vector-p vector)
               (crux:car (prelude:%vector-prop :bitvec vector))
               (:if (prelude:typep vector :vector)
                    (crux:vector-len vector)
                    (crux:cdr (prelude:%vector-prop :disp vector))))
          (prelude:raise vector 'prelude:vector-length "not a vector"))))

(crux:intern prelude "vector-type"
   (:lambda (vector)
     (:if (prelude:vectorp vector)
          (:if (prelude:bit-vector-p vector)
               'bit
               (:if (prelude:typep vector :vector)
                    (crux:vector-type vector)
                    (crux:vector-type (prelude:%vector-prop :base vector))))
          (prelude:raise vector 'prelude:vector-type "not a vector"))))

(crux:intern prelude "vector-ref"
   (:lambda (vector nth)
     (:if (prelude:vectorp vector)
          (:if (prelude:fixnump nth)
               (:if (prelude:%orf (prelude:minusp nth) (crux:less-than (prelude:vector-length vector) nth))
                    (prelude:raise length 'prelude:vector-ref "illegal index")
                    (:if (prelude:bit-vector-p vector)
                         ((:lambda (nth-byte offset)
                            ((:lambda (byte)
                               (:if (prelude:zerop (crux:logand byte (crux:ash 1 (crux:difference 7 offset))))
                                    0
                                    1))
                               (prelude:vector-ref (prelude:%vector-prop :base vector) nth-byte)))
                          (crux:quotient nth 8)
                          (crux:difference nth (crux:product 8 (crux:quotient nth 8))))
                         (:if (prelude:typep vector :vector)
                              (crux:vector-ref vector nth)
                              ((:lambda (base start)
                                 (crux:vector-ref base (crux:sum start nth)))
                               (prelude:%vector-prop :base vector)
                               (crux:car (prelude:%vector-prop :disp vector))))))
                    (prelude:raise nth 'prelude:vector-ref "not a fixnum"))
          (prelude:raise prop 'prelude:vector-ref "not a vector"))))

;;;
;;; write
;;;
(crux:intern prelude "%vector-write"
   (:lambda (vector escape stream)
       (:if (prelude:typep vector :vector)
           (crux:write vector escape stream)
           ((:lambda (length)
               (prelude:format stream "#(" ())
               (crux:fix
                (:lambda (index)
                   (:if (crux:less-than index length)
                        ((:lambda ()
                            (prelude:write (prelude:vector-ref vector index) escape stream)
                            (:if (crux:less-than index (prelude:1- length))
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
(crux:intern prelude "%make-vector"
   (:lambda (vector displace bitvec)
     (prelude:%make-type "%vector"
       `(,(crux:cons :base   vector)
         ,(crux:cons :disp   displace)
         ,(crux:cons :bitvec bitvec)))))

(crux:intern prelude "%specialized-vector-type"
   (:lambda (list)
     ((:lambda (type)
        (:if (prelude:findl (:lambda (stype) (crux:eq type stype)) '(:t :char :fixnum :float))
             (:if (crux:eq type :fixnum)
                  ((:lambda (is-byte)
                     (:if is-byte
                          :byte
                          :fixnum))
                   (prelude:foldl
                    (:lambda (elt acc)
                      (:if (prelude:%andf (prelude:not (prelude:minusp elt)) (crux:less-than elt 256))
                           acc
                           ()))
                      :t
                      list))
                  type)
             :t))
      (prelude:foldl
       (:lambda (elt acc)
          (:if (crux:eq :t acc)
               :t
               (:if (crux:eq acc (crux:type-of elt))
                    acc
                    ())))
      (crux:type-of (crux:car list))
      (crux:cdr list)))))

(crux:intern prelude "%vector-prop"
   (:lambda (prop vector)
      (:if (prelude:findl (:lambda (vec-prop) (crux:eq prop vec-prop)) '(:base :disp :bitvec))
           (crux:cdr (prelude:%type-ref prop vector))
           (prelude:raise prop 'prelude:%vector-prop "not a vector property"))))

(crux:intern prelude "%map-vector"
  (:lambda (fn vector)
    (crux:fix
     (:lambda (nth)
       (:if nth
            (:if (crux:eq nth (crux:vector-len vector))
                 ()
                 ((:lambda ()
                     (prelude:apply fn `(,(crux:vector-ref vector nth)))
                     (crux:sum 1 nth))))
            ()))
     0)))
