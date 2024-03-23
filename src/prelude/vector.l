;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; vectors
;;;
(core:intern :prelude "vectorp"
   (:lambda (vector)
     (:if (core:eq :vector (core:type-of vector))
          :t
          (core:eq 'vector (prelude:type-of vector)))))

(core:intern :prelude "bit-vector-p"
   (:lambda (vector)
     (:if (prelude:%prelude-type-p vector)
          (:if (core:eq 'vector (prelude:type-of vector))
               (prelude:not (prelude:null (prelude:%vector-prop :bitvec vector)))
               ())
          ())))

(core:intern :prelude "vector-displaced-p"
   (:lambda (vector)
     (:if (prelude:%prelude-type-p vector)
          (:if (core:eq 'vector (prelude:type-of vector))
               (prelude:%vector-prop :disp vector)
               ())
          ())))

(core:intern :prelude "make-vector"
   (:lambda (list)
     (:if (prelude:listp list)
          (core:vector (prelude:%specialized-vector-type list) list)
          (prelude:raise list 'prelude:make-vector "not a list"))))

(core:intern :prelude "slice"
   (:lambda (vector start length)
     (:if (prelude:typep vector :vector)
          (:if (prelude:fixnump start)
               (:if (prelude:%orf (prelude:minusp start) (core:fx-lt (prelude:vector-length vector) start))
                    (prelude:raise start 'prelude:slice "illegal start")
                    (:if (prelude:fixnump length)
                         (:if (prelude:%orf (prelude:minusp length) (core:fx-lt (prelude:vector-length vector) length))
                              (prelude:raise length 'prelude:slice "illegal length")
                              (prelude:%make-vector vector `(,start ,@length) ()))
                         (prelude:raise start 'prelude:slice "not a fixnum")))
               (prelude:raise length 'prelude:slice "not a fixnum"))
          (prelude:raise vector 'prelude:slice "not a base vector"))))

(core:intern :prelude "vector-length"
   (:lambda (vector)
     (:if (prelude:vectorp vector)
          (:if (prelude:bit-vector-p vector)
               (core:car (prelude:%vector-prop :bitvec vector))
               (:if (prelude:typep vector :vector)
                    (core:sv-len vector)
                    (core:cdr (prelude:%vector-prop :disp vector))))
          (prelude:raise vector 'prelude:vector-length "not a vector"))))

(core:intern :prelude "vector-type"
   (:lambda (vector)
     (:if (prelude:vectorp vector)
          (:if (prelude:bit-vector-p vector)
               'bit
               (:if (prelude:typep vector :vector)
                    (core:sv-type vector)
                    (core:sv-type (prelude:%vector-prop :base vector))))
          (prelude:raise vector 'prelude:vector-type "not a vector"))))

(core:intern :prelude "vector-ref"
   (:lambda (vector nth)
     (:if (prelude:vectorp vector)
          (:if (prelude:fixnump nth)
               (:if (prelude:%orf (prelude:minusp nth) (core:fx-lt (prelude:vector-length vector) nth))
                    (prelude:raise length 'prelude:vector-ref "illegal index")
                    (:if (prelude:bit-vector-p vector)
                         ((:lambda (nth-byte offset)
                            ((:lambda (byte)
                               (:if (prelude:zerop (core:logand byte (core:ash 1 (core:fx-sub 7 offset))))
                                    0
                                    1))
                               (prelude:vector-ref (prelude:%vector-prop :base vector) nth-byte)))
                          (core:fx-div nth 8)
                          (core:fx-sub nth (core:fx-mul 8 (core:fx-div nth 8))))
                         (:if (prelude:typep vector :vector)
                              (core:sv-ref vector nth)
                              ((:lambda (base start)
                                 (core:sv-ref base (core:fx-add start nth)))
                               (prelude:%vector-prop :base vector)
                               (core:car (prelude:%vector-prop :disp vector))))))
                    (prelude:raise nth 'prelude:vector-ref "not a fixnum"))
          (prelude:raise prop 'prelude:vector-ref "not a vector"))))

;;;
;;; write
;;;
(core:intern :prelude "%vector-write"
   (:lambda (vector escape stream)
       (:if (prelude:typep vector :vector)
           (core:write vector escape stream)
           ((:lambda (length)
               (prelude:format stream "#(" ())
               (core:fix
                (:lambda (index)
                   (:if (core:fx-lt index length)
                        ((:lambda ()
                            (prelude:write (prelude:vector-ref vector index) escape stream)
                            (:if (core:fx-lt index (prelude:1- length))
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
(core:intern :prelude "%make-vector"
   (:lambda (vector displace bitvec)
     (prelude:%make-type "vector"
       `(,(core:cons :base   vector)
         ,(core:cons :disp   displace)
         ,(core:cons :bitvec bitvec)))))

(core:intern :prelude "%specialized-vector-type"
   (:lambda (list)
     ((:lambda (type)
        (:if (prelude:findl (:lambda (stype) (core:eq type stype)) '(:t :char :fixnum :float))
             (:if (core:eq type :fixnum)
                  ((:lambda (is-byte)
                     (:if is-byte
                          :byte
                          :fixnum))
                   (prelude:foldl
                    (:lambda (elt acc)
                      (:if (prelude:%andf (prelude:not (prelude:minusp elt)) (core:fx-lt elt 256))
                           acc
                           ()))
                      :t
                      list))
                  type)
             :t))
      (prelude:foldl
       (:lambda (elt acc)
          (:if (core:eq :t acc)
               :t
               (:if (core:eq acc (core:type-of elt))
                    acc
                    ())))
      (core:type-of (core:car list))
      (core:cdr list)))))

(core:intern :prelude "%vector-prop"
   (:lambda (prop vector)
      (:if (prelude:findl (:lambda (vec-prop) (core:eq prop vec-prop)) '(:base :disp :bitvec))
           (core:cdr (prelude:%type-ref prop vector))
           (prelude:raise prop 'prelude:%vector-prop "not a vector property"))))

(core:intern :prelude "%map-vector"
  (:lambda (fn vector)
    (core:fix
     (:lambda (nth)
       (:if nth
            (:if (core:eq nth (core:sv-len vector))
                 ()
                 ((:lambda ()
                     (prelude:apply fn `(,(core:sv-ref vector nth)))
                     (core:fx-add 1 nth))))
            ()))
     0)))
