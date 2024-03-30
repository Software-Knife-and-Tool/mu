;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; vectors
;;;
(lib:intern :prelude "vectorp"
   (:lambda (vector)
     (:if (lib:eq :vector (lib:type-of vector))
          :t
          (lib:eq 'vector (prelude:type-of vector)))))

(lib:intern :prelude "bit-vector-p"
   (:lambda (vector)
     (:if (prelude:%prelude-type-p vector)
          (:if (lib:eq 'vector (prelude:type-of vector))
               (prelude:not (prelude:null (prelude:%vector-prop :bitvec vector)))
               ())
          ())))

(lib:intern :prelude "vector-displaced-p"
   (:lambda (vector)
     (:if (prelude:%prelude-type-p vector)
          (:if (lib:eq 'vector (prelude:type-of vector))
               (prelude:%vector-prop :disp vector)
               ())
          ())))

(lib:intern :prelude "make-vector"
   (:lambda (list)
     (:if (prelude:listp list)
          (lib:vector (prelude:%specialized-vector-type list) list)
          (prelude:raise list 'prelude:make-vector "not a list"))))

(lib:intern :prelude "slice"
   (:lambda (vector start length)
     (:if (prelude:typep vector :vector)
          (:if (prelude:fixnump start)
               (:if (prelude:%orf (prelude:minusp start) (lib:fx-lt (prelude:vector-length vector) start))
                    (prelude:raise start 'prelude:slice "illegal start")
                    (:if (prelude:fixnump length)
                         (:if (prelude:%orf (prelude:minusp length) (lib:fx-lt (prelude:vector-length vector) length))
                              (prelude:raise length 'prelude:slice "illegal length")
                              (prelude:%make-vector vector `(,start ,@length) ()))
                         (prelude:raise start 'prelude:slice "not a fixnum")))
               (prelude:raise length 'prelude:slice "not a fixnum"))
          (prelude:raise vector 'prelude:slice "not a base vector"))))

(lib:intern :prelude "vector-length"
   (:lambda (vector)
     (:if (prelude:vectorp vector)
          (:if (prelude:bit-vector-p vector)
               (lib:car (prelude:%vector-prop :bitvec vector))
               (:if (prelude:typep vector :vector)
                    (lib:sv-len vector)
                    (lib:cdr (prelude:%vector-prop :disp vector))))
          (prelude:raise vector 'prelude:vector-length "not a vector"))))

(lib:intern :prelude "vector-type"
   (:lambda (vector)
     (:if (prelude:vectorp vector)
          (:if (prelude:bit-vector-p vector)
               'bit
               (:if (prelude:typep vector :vector)
                    (lib:sv-type vector)
                    (lib:sv-type (prelude:%vector-prop :base vector))))
          (prelude:raise vector 'prelude:vector-type "not a vector"))))

(lib:intern :prelude "vector-ref"
   (:lambda (vector nth)
     (:if (prelude:vectorp vector)
          (:if (prelude:fixnump nth)
               (:if (prelude:%orf (prelude:minusp nth) (lib:fx-lt (prelude:vector-length vector) nth))
                    (prelude:raise length 'prelude:vector-ref "illegal index")
                    (:if (prelude:bit-vector-p vector)
                         ((:lambda (nth-byte offset)
                            ((:lambda (byte)
                               (:if (prelude:zerop (lib:logand byte (lib:ash 1 (lib:fx-sub 7 offset))))
                                    0
                                    1))
                               (prelude:vector-ref (prelude:%vector-prop :base vector) nth-byte)))
                          (lib:fx-div nth 8)
                          (lib:fx-sub nth (lib:fx-mul 8 (lib:fx-div nth 8))))
                         (:if (prelude:typep vector :vector)
                              (lib:sv-ref vector nth)
                              ((:lambda (base start)
                                 (lib:sv-ref base (lib:fx-add start nth)))
                               (prelude:%vector-prop :base vector)
                               (lib:car (prelude:%vector-prop :disp vector))))))
                    (prelude:raise nth 'prelude:vector-ref "not a fixnum"))
          (prelude:raise prop 'prelude:vector-ref "not a vector"))))

;;;
;;; write
;;;
(lib:intern :prelude "%vector-write"
   (:lambda (vector escape stream)
       (:if (prelude:typep vector :vector)
           (lib:write vector escape stream)
           ((:lambda (length)
               (prelude:format stream "#(" ())
               (lib:fix
                (:lambda (index)
                   (:if (lib:fx-lt index length)
                        ((:lambda ()
                            (prelude:write (prelude:vector-ref vector index) escape stream)
                            (:if (lib:fx-lt index (prelude:1- length))
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
(lib:intern :prelude "%make-vector"
   (:lambda (vector displace bitvec)
     (prelude:%make-type "vector"
       `(,(lib:cons :base   vector)
         ,(lib:cons :disp   displace)
         ,(lib:cons :bitvec bitvec)))))

(lib:intern :prelude "%specialized-vector-type"
   (:lambda (list)
     ((:lambda (type)
        (:if (prelude:findl (:lambda (stype) (lib:eq type stype)) '(:t :char :fixnum :float))
             (:if (lib:eq type :fixnum)
                  ((:lambda (is-byte)
                     (:if is-byte
                          :byte
                          :fixnum))
                   (prelude:foldl
                    (:lambda (elt acc)
                      (:if (prelude:%andf (prelude:not (prelude:minusp elt)) (lib:fx-lt elt 256))
                           acc
                           ()))
                      :t
                      list))
                  type)
             :t))
      (prelude:foldl
       (:lambda (elt acc)
          (:if (lib:eq :t acc)
               :t
               (:if (lib:eq acc (lib:type-of elt))
                    acc
                    ())))
      (lib:type-of (lib:car list))
      (lib:cdr list)))))

(lib:intern :prelude "%vector-prop"
   (:lambda (prop vector)
      (:if (prelude:findl (:lambda (vec-prop) (lib:eq prop vec-prop)) '(:base :disp :bitvec))
           (lib:cdr (prelude:%type-ref prop vector))
           (prelude:raise prop 'prelude:%vector-prop "not a vector property"))))

(lib:intern :prelude "%map-vector"
  (:lambda (fn vector)
    (lib:fix
     (:lambda (nth)
       (:if nth
            (:if (lib:eq nth (lib:sv-len vector))
                 ()
                 ((:lambda ()
                     (prelude:apply fn `(,(lib:sv-ref vector nth)))
                     (lib:fx-add 1 nth))))
            ()))
     0)))
