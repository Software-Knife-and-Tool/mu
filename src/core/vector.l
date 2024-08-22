;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; vectors
;;;

;;;
;;; core type
;;;
(core:define-type "%vector"
    '((:base    . :vector)
      (:disp    . :listp)
      (:bitvec  . :listp)))

(mu:intern core "%vector-prop"
   (:lambda (prop vector)
      (:if (core:findl (:lambda (vec-prop) (mu:eq prop vec-prop)) '(:base :disp :bitvec))
           (mu:cdr (core:%type-ref prop vector))
           (core:raise prop 'core:%vector-prop "not a vector property"))))

(mu:intern core "%make-vector"
   (:lambda (vector displace bitvec)
     (core:%make-type "%vector"
       `(,(mu:cons :base   vector)
         ,(mu:cons :disp   displace)
         ,(mu:cons :bitvec bitvec)))))

;;;
;;; predicates
;;;
(mu:intern core "vectorp"
   (:lambda (vector)
     (:if (mu:eq :vector (mu:type-of vector))
          :t
          (mu:eq '%vector (core:type-of vector)))))

(mu:intern core "bit-vector-p"
   (:lambda (vector)
     (:if (core:%core-type-p vector)
          (:if (mu:eq '%vector (core:type-of vector))
               (core:not (core:null (core:%vector-prop :bitvec vector)))
               ())
          ())))

(mu:intern core "vector-displaced-p"
   (:lambda (vector)
     (:if (core:%core-type-p vector)
          (:if (mu:eq '%vector (core:type-of vector))
               (core:%vector-prop :disp vector)
               ())
          ())))

;;;
;;; constructors
;;;
(mu:intern core "make-vector"
   (:lambda (list)
     (:if (core:listp list)
          (mu:make-vector (core:%specialized-vector-type list) list)
          (core:raise list 'core:make-vector "not a list"))))

(mu:intern core "vector-slice"
   (:lambda (vector start length)
     (:if (core:typep vector :vector)
          (:if (core:fixnump start)
               (:if (core:%orf (core:minusp start) (mu:less-than (core:vector-length vector) start))
                    (core:raise start 'core:vector-slice "illegal start")
                    (:if (core:fixnump length)
                         (:if (core:%orf (core:minusp length) (mu:less-than (core:vector-length vector) length))
                              (core:raise length 'core:vector-slice "illegal length")
                              (core:%make-vector vector `(,start ,@length) ()))
                         (core:raise start 'core:vector-slice "not a fixnum")))
               (core:raise length 'core:vector-slice "not a fixnum"))
          (core:raise vector 'core:vector-slice "not a base vector"))))

(mu:intern core "vector-length"
   (:lambda (vector)
     (:if (core:vectorp vector)
          (:if (core:bit-vector-p vector)
               (mu:car (core:%vector-prop :bitvec vector))
               (:if (core:typep vector :vector)
                    (mu:vector-len vector)
                    (mu:cdr (core:%vector-prop :disp vector))))
          (core:raise vector 'core:vector-length "not a vector"))))

(mu:intern core "vector-type"
   (:lambda (vector)
     (:if (core:vectorp vector)
          (:if (core:bit-vector-p vector)
               'bit
               (:if (core:typep vector :vector)
                    (mu:vector-type vector)
                    (mu:vector-type (core:%vector-prop :base vector))))
          (core:raise vector 'core:vector-type "not a vector"))))

(mu:intern core "vector-ref"
   (:lambda (vector nth)
     (:if (core:vectorp vector)
          (:if (core:fixnump nth)
               (:if (core:%orf (core:minusp nth) (mu:less-than (core:vector-length vector) nth))
                    (core:raise length 'core:vector-ref "illegal index")
                    (:if (core:bit-vector-p vector)
                         ((:lambda (nth-byte offset)
                            ((:lambda (byte)
                               (:if (core:zerop (mu:logand byte (mu:ash 1 (mu:difference 7 offset))))
                                    0
                                    1))
                               (core:vector-ref (core:%vector-prop :base vector) nth-byte)))
                          (mu:quotient nth 8)
                          (mu:difference nth (mu:product 8 (mu:quotient nth 8))))
                         (:if (core:typep vector :vector)
                              (mu:svref vector nth)
                              ((:lambda (base start)
                                 (mu:svref base (mu:sum start nth)))
                               (core:%vector-prop :base vector)
                               (mu:car (core:%vector-prop :disp vector))))))
                    (core:raise nth 'core:vector-ref "not a fixnum"))
          (core:raise prop 'core:vector-ref "not a vector"))))

;;;
;;; write
;;;
(mu:intern core "%vector-write"
   (:lambda (vector escape stream)
       (:if (core:typep vector :vector)
           (mu:write vector escape stream)
           ((:lambda (length)
               (core:format stream "#(" ())
               (mu:fix
                (:lambda (index)
                   (:if (mu:less-than index length)
                        ((:lambda ()
                            (core:write (core:vector-ref vector index) escape stream)
                            (:if (mu:less-than index (core:1- length))
                                 (core:format stream " " ())
                                 ())
                            (core:1+ index)))
                        index))
                0)
               (core:format stream ")" ()))
            (core:vector-length vector)))
      ()))

;;;
;;; implementations
;;;
(mu:intern core "%specialized-vector-type"
   (:lambda (list)
     ((:lambda (type)
        (:if (core:findl (:lambda (stype) (mu:eq type stype)) '(:t :char :fixnum :float))
             (:if (mu:eq type :fixnum)
                  ((:lambda (is-byte)
                     (:if is-byte
                          :byte
                          :fixnum))
                   (core:%foldl
                    (:lambda (elt acc)
                      (:if (core:%andf (core:not (core:minusp elt)) (mu:less-than elt 256))
                           acc
                           ()))
                      :t
                      list))
                  type)
             :t))
      (core:%foldl
       (:lambda (elt acc)
          (:if (mu:eq :t acc)
               :t
               (:if (mu:eq acc (mu:type-of elt))
                    acc
                    ())))
      (mu:type-of (mu:car list))
      (mu:cdr list)))))

(mu:intern core "%map-vector"
  (:lambda (fn vector)
    (mu:fix
     (:lambda (nth)
       (:if nth
            (:if (mu:eq nth (mu:vector-len vector))
                 ()
                 ((:lambda ()
                     (core:%apply fn `(,(mu:svref vector nth)))
                     (mu:sum 1 nth))))
            ()))
     0)))
