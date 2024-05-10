;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; inspect
;;;
(core:intern prelude "%inspect-function"
   (:lambda (obj)
      ((:lambda (view)
          ((:lambda (tags)
             (core:make-struct :inspect `(function ,(core:heap-size obj) ,tags)))
           `(,(core:cons :arity (core:vector-ref view 0))
             ,(core:cons :body (core:vector-ref view 1)))))
       (core:view obj))))

(core:intern prelude "%inspect-type"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)
            (core:make-struct :inspect `(:<type> ,(core:heap-size obj) ,tags)))
          `(,(core:cons :type (core:vector-ref (core:struct-vec obj) 0))
            ,(core:cons :values (core:vector-ref (core:struct-vec obj) 1)))))
      (core:view obj))))

(core:intern prelude "%inspect-struct"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (core:make-struct :inspect `(struct ,(core:heap-size obj) ,tags)))
          `(,(core:cons :type (core:struct-type obj))
            ,(core:cons :values (core:struct-vec obj)))))
      (core:view obj))))

(core:intern prelude "%inspect-symbol"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (core:make-struct :inspect `(symbol ,(core:heap-size obj) ,tags)))
          `(,(core:cons :ns (core:vector-ref view 0))
            ,(core:cons :name (core:vector-ref view 1))
            ,(core:cons :value (core:vector-ref view 2)))))
      (core:view obj))))

(core:intern prelude "%inspect-const"
   (:lambda (obj)
     (core:make-struct :inspect `(,(prelude:type-of obj) ,(core:heap-size obj) ()))))

(core:intern prelude "%inspect-vector"
   (:lambda (obj)
     ((:lambda (tags)          
         (core:make-struct :inspect `(vector ,(core:heap-size obj) ,tags)))
      `(,(core:cons :type (core:vector-type obj))
        ,(core:cons :length (core:vector-len obj))))))

(core:intern prelude "%inspect-cons"
   (:lambda (obj)
     ((:lambda (tags)          
         (core:make-struct :inspect `(cons ,(core:heap-size obj) ,tags)))
      `(,(core:cons :car (core:car obj))
        ,(core:cons :cdr (core:cdr obj))))))

(core:intern prelude "%inspect"
   (:lambda (value)
      (core:fix
       (:lambda (loop)
          (:if (prelude:structp loop)
               loop
               (:if (prelude:null loop)
                    (core:make-struct :inspect `(unknown 0 ()))
                    ((:lambda (predicate fn)
                       (:if (core:apply predicate `(,value))
                            (core:apply fn `(,value))
                            (core:cdr loop)))
                     (core:car (core:car loop))
                     (core:cdr (core:car loop))))))
       `(,(core:cons prelude:%prelude-type-p prelude:%inspect-type)
         ,(core:cons prelude:charp prelude:%inspect-const)
         ,(core:cons prelude:consp prelude:%inspect-cons)
         ,(core:cons prelude:fixnump prelude:%inspect-const)
         ,(core:cons prelude:floatp prelude:%inspect-const)
         ,(core:cons prelude:functionp prelude:%inspect-function)
         ,(core:cons prelude:structp prelude:%inspect-struct)
         ,(core:cons prelude:symbolp prelude:%inspect-symbol)
         ,(core:cons prelude:vectorp prelude:%inspect-vector)))))
