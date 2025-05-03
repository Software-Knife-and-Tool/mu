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

(mu:intern core "vector-prop"
   (:lambda (prop vector)
      (:if (core:%find-if (:lambda (vec-prop) (mu:eq prop vec-prop)) '(:base :disp :bitvec))
           (core:%vector-prop prop vector)
           (core:%raise prop 'core:vector-prop "not a vector property"))))

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
          (core:%raise list 'core:make-vector "not a list"))))

(mu:intern core "vector-slice"
   (:lambda (vector start length)
     (:if (core:vectorp vector)
          (:if (core:fixnump start)
               (:if (core:%or (core:minusp start) (mu:less-than (core:vector-length vector) start))
                    (core:%raise start 'core:vector-slice "illegal start")
                    (:if (core:fixnump length)
                         (:if (core:%or (core:minusp length) (mu:less-than (core:vector-length vector) length))
                              (core:%raise length 'core:vector-slice "illegal length")
                              (core:%make-vector vector (mu:cons start length) ()))
                         (core:%raise start 'core:vector-slice "not a fixnum")))
               (core:%raise length 'core:vector-slice "not a fixnum"))
          (core:%raise vector 'core:vector-slice "not a base vector"))))

(mu:intern core "vector-length"
   (:lambda (vector)
     (:if (core:vectorp vector)
          (:if (core:bit-vector-p vector)
               (mu:car (core:%vector-prop :bitvec vector))
               (:if (mu:eq :vector (mu:type-of vector))
                    (mu:vector-length vector)
                    (mu:cdr (core:%vector-prop :disp vector))))
          (core:%raise vector 'core:vector-length "not a vector"))))

(mu:intern core "vector-type"
   (:lambda (vector)
     (:if (core:vectorp vector)
          (:if (core:bit-vector-p vector)
               'bit
               (:if (mu:eq :vector (mu:type-of vector))
                    (mu:vector-type vector)
                    (mu:vector-type (core:%vector-prop :base vector))))
          (core:%raise vector 'core:vector-type "not a vector"))))

(mu:intern core "vector-ref"
   (:lambda (vector nth)
     (:if (core:vectorp vector)
          (:if (core:fixnump nth)
               (:if (core:%or (core:minusp nth) (mu:less-than (core:vector-length vector) nth))
                    (core:%raise length 'core:vector-ref "illegal index")
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
                    (core:%raise nth 'core:vector-ref "not a fixnum"))
          (core:%raise prop 'core:vector-ref "not a vector"))))

;;;
;;; write
;;;
(mu:intern core "%vector-write"
   (:lambda (vector escape stream)
       (:if (core:vectorp vector)
           (mu:write vector escape stream)
           ((:lambda (length)
               (core:%format stream "#(" ())
               (mu:fix
                (:lambda (index)
                   (:if (mu:less-than index length)
                        ((:lambda ()
                            (core:write (core:vector-ref vector index) escape stream)
                            (:if (mu:less-than index (mu:sub length 1))
                                 (core:%format stream " " ())
                                 ())
                            (mu:add index 1)))
                        index))
                0)
               (core:%format stream ")" ()))
            (core:vector-length vector)))
      ()))

;;;
;;; implementations
;;;
(mu:intern core "%specialized-vector-type"
   (:lambda (list)
     ((:lambda (type)
        (:if (core:%find-if (:lambda (stype) (mu:eq type stype)) '(:t :char :fixnum :float))
             (:if (mu:eq type :fixnum)
                  ((:lambda (is-byte)
                     (:if is-byte
                          :byte
                          :fixnum))
                   (core:%foldl
                    (:lambda (elt acc)
                      (:if (core:%and (core:null (core:minusp elt)) (mu:less-than elt 256))
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
            (:if (mu:eq nth (mu:vector-length vector))
                 ()
                 (core:%prog2
                  (core:apply fn (mu:cons (mu:svref vector nth) ()))
                  (mu:add 1 nth)))
            ()))
     0)))

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

;;;
;;; strings
;;;
(mu:intern core "stringp"
   (:lambda (string)
      (:if (mu:eq (mu:type-of string) :vector)
           (mu:eq :char (mu:vector-type string))
           (:if (core:%typep string)
                (:if (mu:eq "vector" (mu:svref (mu:struct-vec string) 0))
                     (mu:eq :char (mu:vector-type (core:%vector-prop :base string)))
                     ())
                ()))))

(mu:intern core "%string-position"
  (:lambda (ch str)
    (:if (mu:eq 0 (mu:vector-length str))
         ()
         ((:lambda (len)
            (mu:fix
             (:lambda (loop)
               (:if (core:numberp loop)
                    (:if (mu:less-than (mu:sub len 1) loop)
                         ()
                         (:if (mu:eq ch (mu:svref str loop))
                              loop
                              (mu:add 1 loop)))
                    loop))
             0))
          (mu:vector-length str)))))

(mu:intern core "%substr"
  (:lambda (str start end)
    ((:lambda (substr)
       (mu:fix
        (:lambda (nth)
          (:if (core:numberp nth)
               (:if (mu:eq nth (mu:vector-length str))
                    ()
                    (:if (mu:less-than nth (mu:add 1 end))
                         (core:%prog2
                            (mu:write (mu:svref str nth) () substr)
                            (mu:add 1 nth))
                         ()))
               nth))
        start)
       (mu:get-string substr))
     (mu:open :string :output "" :t))))

(mu:intern core "%string="
   (:lambda (s1 s2)
     (:if (mu:eq (mu:vector-length s1) (mu:vector-length s2))
          (mu:fix
           (:lambda (loop)
             (:if (core:fixnump loop)
                  (:if (mu:less-than loop (mu:vector-length s1))
                       (:if (mu:eq (mu:svref s1 loop) (mu:svref s2 loop))
                            (mu:add 1 loop)
                            ())
                       :t)
                  loop))
           0)
          ())))

(mu:intern core "%string-concat"
   (:lambda (strings)
     ((:lambda (concat)
        (mu:fix
         (:lambda (list)
           (:if list
                ((:lambda (string)
                   (mu:fix
                    (:lambda (index)
                      (:if (mu:less-than index (mu:vector-length string))
                           (core:%prog2
                              (mu:write-char (mu:svref string index) concat)
                              (mu:add index 1))
                           index))
                    0)
                   (mu:cdr list))
                 (mu:car list))
                list))
         strings)
        (mu:get-string concat))
        (mu:open :string :output "" :t))))
