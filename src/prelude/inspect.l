;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; inspect
;;;
(crux:intern prelude "%inspect-function"
   (:lambda (obj)
      ((:lambda (view)
          ((:lambda (tags)
             (crux:make-struct :inspect `(function ,(crux:heap-size obj) ,tags)))
           `(,(crux:cons :arity (crux:vector-ref view 0))
             ,(crux:cons :body (crux:vector-ref view 1)))))
       (crux:view obj))))

(crux:intern prelude "%inspect-type"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)
            (crux:make-struct :inspect `(:<type> ,(crux:heap-size obj) ,tags)))
          `(,(crux:cons :type (crux:vector-ref (crux:struct-vec obj) 0))
            ,(crux:cons :values (crux:vector-ref (crux:struct-vec obj) 1)))))
      (crux:view obj))))

(crux:intern prelude "%inspect-struct"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (crux:make-struct :inspect `(struct ,(crux:heap-size obj) ,tags)))
          `(,(crux:cons :type (crux:struct-type obj))
            ,(crux:cons :values (crux:struct-vec obj)))))
      (crux:view obj))))

(crux:intern prelude "%inspect-symbol"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (crux:make-struct :inspect `(symbol ,(crux:heap-size obj) ,tags)))
          `(,(crux:cons :ns (crux:vector-ref view 0))
            ,(crux:cons :name (crux:vector-ref view 1))
            ,(crux:cons :value (crux:vector-ref view 2)))))
      (crux:view obj))))

(crux:intern prelude "%inspect-const"
   (:lambda (obj)
     (crux:make-struct :inspect `(,(prelude:type-of obj) ,(crux:heap-size obj) ()))))

(crux:intern prelude "%inspect-vector"
   (:lambda (obj)
     ((:lambda (tags)          
         (crux:make-struct :inspect `(vector ,(crux:heap-size obj) ,tags)))
      `(,(crux:cons :type (crux:vector-type obj))
        ,(crux:cons :length (crux:vector-len obj))))))

(crux:intern prelude "%inspect-cons"
   (:lambda (obj)
     ((:lambda (tags)          
         (crux:make-struct :inspect `(cons ,(crux:heap-size obj) ,tags)))
      `(,(crux:cons :car (crux:car obj))
        ,(crux:cons :cdr (crux:cdr obj))))))

(crux:intern prelude "%inspect"
   (:lambda (value)
      (crux:fix
       (:lambda (loop)
          (:if (prelude:structp loop)
               loop
               (:if (prelude:null loop)
                    (crux:make-struct :inspect `(unknown 0 ()))
                    ((:lambda (predicate fn)
                       (:if (crux:apply predicate `(,value))
                            (crux:apply fn `(,value))
                            (crux:cdr loop)))
                     (crux:car (crux:car loop))
                     (crux:cdr (crux:car loop))))))
       `(,(crux:cons prelude:%prelude-type-p prelude:%inspect-type)
         ,(crux:cons prelude:charp prelude:%inspect-const)
         ,(crux:cons prelude:consp prelude:%inspect-cons)
         ,(crux:cons prelude:fixnump prelude:%inspect-const)
         ,(crux:cons prelude:floatp prelude:%inspect-const)
         ,(crux:cons prelude:functionp prelude:%inspect-function)
         ,(crux:cons prelude:structp prelude:%inspect-struct)
         ,(crux:cons prelude:symbolp prelude:%inspect-symbol)
         ,(crux:cons prelude:vectorp prelude:%inspect-vector)))))
