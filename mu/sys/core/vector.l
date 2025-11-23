;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; vectors
;;;

;;;
;;; vector deftype
;;;
(core:%deftype "%vector"
    '((:base    . :vector)
      (:disp    . :listp)
      (:bitvec  . :listp)))

(mu:intern core "%vector-prop"
   (:lambda (prop vector)
     (mu:cdr (core:%type-prop prop vector))))

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
     (:if (core:%typep vector)
          (:if (mu:eq '%vector (core:type-of vector))
               (core:null (core:null (core:%vector-prop :bitvec vector)))
               ())
          ())))

(mu:intern core "vector-displaced-p"
   (:lambda (vector)
     (:if (core:%typep vector)
          (:if (mu:eq '%vector (core:type-of vector))
               (core:%vector-prop :disp vector)
               ())
          ())))

;;;
;;; functions
;;;
(mu:intern core "make-vector"
   (:lambda (list)
     (:if (core:listp list)
          (mu:make-vector (core:%specialized-vector-type list) list)
          (mu:raise-from list 'core:make-vector :type))))

(mu:intern core "vector-slice"
   (:lambda (vector start length)
     (:if (core:vectorp vector)
          (:if (core:fixnump start)
               (:if (core:%or (core:minusp start) (mu:less-than (core:vector-length vector) start))
                    (mu:raise-from start 'core:vector-slice :range)
                    (:if (core:fixnump length)
                         (:if (core:%or (core:minusp length) (mu:less-than (core:vector-length vector) length))
                              (mu:raise-from length 'core:vector-slice :range)
                              (core:%make-vector vector (mu:cons start length) ()))
                         (mu:raise-from start 'core:vector-slice :type)))
               (mu:raise-from length 'core:vector-slice :type))
          (mu:raise-from vector 'core:vector-slice :type))))

(mu:intern core "vector-length"
   (:lambda (vector)
     (:if (core:vectorp vector)
          (:if (core:bit-vector-p vector)
               (mu:car (core:%vector-prop :bitvec vector))
               (:if (mu:eq :vector (mu:type-of vector))
                    (mu:vector-length vector)
                    (mu:cdr (core:%vector-prop :disp vector))))
          (mu:raise-from vector 'core:vector-length :type))))

(mu:intern core "vector-type"
   (:lambda (vector)
     (:if (core:vectorp vector)
          (:if (core:bit-vector-p vector)
               'bit
               (:if (mu:eq :vector (mu:type-of vector))
                    (mu:vector-type vector)
                    (mu:vector-type (core:%vector-prop :base vector))))
          (mu:raise-from vector 'core:vector-type :type))))

(mu:intern core "vector-ref"
   (:lambda (vector nth)
     (:if (core:vectorp vector)
          (:if (core:fixnump nth)
               (:if (core:%or (core:minusp nth) (mu:less-than (core:vector-length vector) nth))
                    (mu:raise-from nth 'core:vector-ref :range)
                    (:if (core:bit-vector-p vector)
                         ((:lambda (nth-byte offset)
                            ((:lambda (byte)
                               (:if (mu:eq 0 (mu:logand byte (mu:ash 1 (mu:sub 7 offset))))
                                    0
                                    1))
                               (core:vector-ref (core:%vector-prop :base vector) nth-byte)))
                          (mu:div nth 8)
                          (mu:sub nth (mu:mul 8 (mu:div nth 8))))
                         (:if (mu:eq :vector (mu:type-of vector))
                              (mu:svref vector nth)
                              ((:lambda (base start)
                                 (mu:svref base (mu:add start nth)))
                               (core:%vector-prop :base vector)
                               (mu:car (core:%vector-prop :disp vector))))))
                    (mu:raise-from nth 'core:vector-ref :type))
          (mu:raise-from vector 'core:vector-ref :type))))

;;;
;;; write
;;;
(mu:intern core "%vector-write"
   (:lambda (vector escape stream)
       (:if (core:vectorp vector)
           (mu:write vector escape stream)
           ((:lambda (length)
               (core:write "#(" () stream)
               (mu:fix
                (:lambda (index)
                   (:if (mu:less-than index length)
                        ((:lambda ()
                            (core:write (core:vector-ref vector index) escape stream)
                            (:if (mu:less-than index (mu:sub length 1))
                                 (core:write " " () stream)
                                 ())
                            (mu:add index 1)))
                        index))
                0)
               (core:write ")" () stream))
            (core:vector-length vector)))
      ()))

;;;
;;; implementations
;;;
(mu:intern core "%specialized-vector-type"
   (:lambda (list)
     ((:lambda (type)
        (:if (core:find-if (:lambda (stype) (mu:eq type stype)) '(:t :char :fixnum :float))
             (:if (mu:eq type :fixnum)
                  ((:lambda (is-byte)
                     (:if is-byte
                          :byte
                          :fixnum))
                   (core:foldl
                    (:lambda (elt acc)
                      (:if (core:%and (core:null (core:minusp elt)) (mu:less-than elt 256))
                           acc
                           ()))
                      :t
                      list))
                  type)
             :t))
      (core:foldl
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
            (:if (mu:eq nth (mu:vector-length vector))
                 ()
                 (core:%prog2
                  (core:apply fn (mu:cons (mu:svref vector nth) ()))
                  (mu:add 1 nth)))
            ()))
     0)))

(mu:intern core "%vector-position"
  (:lambda (el str)
    (:if (mu:eq 0 (mu:vector-length str))
         ()
         ((:lambda (len)
            (mu:fix
             (:lambda (loop)
               (:if (core:numberp loop)
                    (:if (mu:less-than (mu:sub len 1) loop)
                         ()
                         (:if (mu:eq el (mu:svref str loop))
                              loop
                              (mu:add 1 loop)))
                    loop))
             0))
          (mu:vector-length str)))))

(mu:intern core "%vector-substr"
  (:lambda (vec start end)
    ((:lambda (subvec)
       (mu:fix
        (:lambda (nth)
          (:if (core:numberp nth)
               (:if (mu:eq nth (mu:vector-length vec))
                    ()
                    (:if (mu:less-than nth (mu:add 1 end))
                         (core:%prog2
                            (mu:write (mu:svref vec nth) () subvec)
                            (mu:add 1 nth))
                         ()))
               nth))
        start)
       (mu:get-string subvec))
     (mu:open :string :output "" :t))))

(mu:intern core "%vector="
   (:lambda (v1 v2)
     (:if (mu:eq (mu:vector-length v1) (mu:vector-length v2))
          (mu:fix
           (:lambda (loop)
             (:if (core:fixnump loop)
                  (:if (mu:less-than loop (mu:vector-length v1))
                       (:if (mu:eq (mu:svref v1 loop) (mu:svref v2 loop))
                            (mu:add 1 loop)
                            ())
                       :t)
                  loop))
           0)
          ())))

;;; shouldn't this be a coerce?
(mu:intern core "%vector-to-list"
   (:lambda (vector)
     (:if (mu:eq 0 (mu:vector-length vector))
          ()
          (mu:cdr
           (mu:fix
            (:lambda (loop)
              ((:lambda (nth list)
                 (:if (core:fixnump nth)
                     (:if (mu:less-than nth (mu:vector-length vector))
                          (mu:cons (mu:add 1 nth) (core:%append `(,list ,(mu:cons (mu:svref vector nth) ()))))
                          (mu:cons () list))
                     loop))
               (mu:car loop)
               (mu:cdr loop)))
            (mu:cons 0 ()))))))
