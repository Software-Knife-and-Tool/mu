;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; vectors
;;;
(libcore:intern :prelude "vectorp"
   (:lambda (vector)
     (:if (libcore:eq :vector (libcore:type-of vector))
          :t
          (libcore:eq 'vector (prelude:type-of vector)))))

(libcore:intern :prelude "bit-vector-p"
   (:lambda (vector)
     (:if (prelude:%prelude-type-p vector)
          (:if (libcore:eq 'vector (prelude:type-of vector))
               (prelude:not (prelude:null (prelude:%vector-prop :bitvec vector)))
               ())
          ())))

(libcore:intern :prelude "vector-displaced-p"
   (:lambda (vector)
     (:if (prelude:%prelude-type-p vector)
          (:if (libcore:eq 'vector (prelude:type-of vector))
               (prelude:%vector-prop :disp vector)
               ())
          ())))

(libcore:intern :prelude "make-vector"
   (:lambda (list)
     (:if (prelude:listp list)
          (libcore:vector (prelude:%specialized-vector-type list) list)
          (prelude:raise list 'prelude:make-vector "not a list"))))

(libcore:intern :prelude "slice"
   (:lambda (vector start length)
     (:if (prelude:typep vector :vector)
          (:if (prelude:fixnump start)
               (:if (prelude:%orf (prelude:minusp start) (libcore:fx-lt (prelude:vector-length vector) start))
                    (prelude:raise start 'prelude:slice "illegal start")
                    (:if (prelude:fixnump length)
                         (:if (prelude:%orf (prelude:minusp length) (libcore:fx-lt (prelude:vector-length vector) length))
                              (prelude:raise length 'prelude:slice "illegal length")
                              (prelude:%make-vector vector `(,start ,@length) ()))
                         (prelude:raise start 'prelude:slice "not a fixnum")))
               (prelude:raise length 'prelude:slice "not a fixnum"))
          (prelude:raise vector 'prelude:slice "not a base vector"))))

(libcore:intern :prelude "vector-length"
   (:lambda (vector)
     (:if (prelude:vectorp vector)
          (:if (prelude:bit-vector-p vector)
               (libcore:car (prelude:%vector-prop :bitvec vector))
               (:if (prelude:typep vector :vector)
                    (libcore:sv-len vector)
                    (libcore:cdr (prelude:%vector-prop :disp vector))))
          (prelude:raise vector 'prelude:vector-length "not a vector"))))

(libcore:intern :prelude "vector-type"
   (:lambda (vector)
     (:if (prelude:vectorp vector)
          (:if (prelude:bit-vector-p vector)
               'bit
               (:if (prelude:typep vector :vector)
                    (libcore:sv-type vector)
                    (libcore:sv-type (prelude:%vector-prop :base vector))))
          (prelude:raise vector 'prelude:vector-type "not a vector"))))

(libcore:intern :prelude "vector-ref"
   (:lambda (vector nth)
     (:if (prelude:vectorp vector)
          (:if (prelude:fixnump nth)
               (:if (prelude:%orf (prelude:minusp nth) (libcore:fx-lt (prelude:vector-length vector) nth))
                    (prelude:raise length 'prelude:vector-ref "illegal index")
                    (:if (prelude:bit-vector-p vector)
                         ((:lambda (nth-byte offset)
                            ((:lambda (byte)
                               (:if (prelude:zerop (libcore:logand byte (libcore:ash 1 (libcore:fx-sub 7 offset))))
                                    0
                                    1))
                               (prelude:vector-ref (prelude:%vector-prop :base vector) nth-byte)))
                          (libcore:fx-div nth 8)
                          (libcore:fx-sub nth (libcore:fx-mul 8 (libcore:fx-div nth 8))))
                         (:if (prelude:typep vector :vector)
                              (libcore:sv-ref vector nth)
                              ((:lambda (base start)
                                 (libcore:sv-ref base (libcore:fx-add start nth)))
                               (prelude:%vector-prop :base vector)
                               (libcore:car (prelude:%vector-prop :disp vector))))))
                    (prelude:raise nth 'prelude:vector-ref "not a fixnum"))
          (prelude:raise prop 'prelude:vector-ref "not a vector"))))

;;;
;;; write
;;;
(libcore:intern :prelude "%vector-write"
   (:lambda (vector escape stream)
       (:if (prelude:typep vector :vector)
           (libcore:write vector escape stream)
           ((:lambda (length)
               (prelude:format stream "#(" ())
               (libcore:fix
                (:lambda (index)
                   (:if (libcore:fx-lt index length)
                        ((:lambda ()
                            (prelude:write (prelude:vector-ref vector index) escape stream)
                            (:if (libcore:fx-lt index (prelude:1- length))
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
(libcore:intern :prelude "%make-vector"
   (:lambda (vector displace bitvec)
     (prelude:%make-type "vector"
       `(,(libcore:cons :base   vector)
         ,(libcore:cons :disp   displace)
         ,(libcore:cons :bitvec bitvec)))))

(libcore:intern :prelude "%specialized-vector-type"
   (:lambda (list)
     ((:lambda (type)
        (:if (prelude:findl (:lambda (stype) (libcore:eq type stype)) '(:t :char :fixnum :float))
             (:if (libcore:eq type :fixnum)
                  ((:lambda (is-byte)
                     (:if is-byte
                          :byte
                          :fixnum))
                   (prelude:foldl
                    (:lambda (elt acc)
                      (:if (prelude:%andf (prelude:not (prelude:minusp elt)) (libcore:fx-lt elt 256))
                           acc
                           ()))
                      :t
                      list))
                  type)
             :t))
      (prelude:foldl
       (:lambda (elt acc)
          (:if (libcore:eq :t acc)
               :t
               (:if (libcore:eq acc (libcore:type-of elt))
                    acc
                    ())))
      (libcore:type-of (libcore:car list))
      (libcore:cdr list)))))

(libcore:intern :prelude "%vector-prop"
   (:lambda (prop vector)
      (:if (prelude:findl (:lambda (vec-prop) (libcore:eq prop vec-prop)) '(:base :disp :bitvec))
           (libcore:cdr (prelude:%type-ref prop vector))
           (prelude:raise prop 'prelude:%vector-prop "not a vector property"))))

(libcore:intern :prelude "%map-vector"
  (:lambda (fn vector)
    (libcore:fix
     (:lambda (nth)
       (:if nth
            (:if (libcore:eq nth (libcore:sv-len vector))
                 ()
                 ((:lambda ()
                     (prelude:apply fn `(,(libcore:sv-ref vector nth)))
                     (libcore:fx-add 1 nth))))
            ()))
     0)))
