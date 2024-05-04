;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; inspect
;;;
(lib:intern prelude "%inspect-function"
   (:lambda (obj)
      ((:lambda (view)
          ((:lambda (tags)
             (lib:struct :inspect `(function ,(lib:heap-size obj) ,tags)))
           `(,(lib:cons :arity (lib:sv-ref view 0))
             ,(lib:cons :body (lib:sv-ref view 1)))))
       (lib:view obj))))

(lib:intern prelude "%inspect-type"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)
            (lib:struct :inspect `(:<type> ,(lib:heap-size obj) ,tags)))
          `(,(lib:cons :type (lib:sv-ref (lib:st-vec obj) 0))
            ,(lib:cons :values (lib:sv-ref (lib:st-vec obj) 1)))))
      (lib:view obj))))

(lib:intern prelude "%inspect-struct"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (lib:struct :inspect `(struct ,(lib:heap-size obj) ,tags)))
          `(,(lib:cons :type (lib:st-type obj))
            ,(lib:cons :values (lib:st-vec obj)))))
      (lib:view obj))))

(lib:intern prelude "%inspect-symbol"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (lib:struct :inspect `(symbol ,(lib:heap-size obj) ,tags)))
          `(,(lib:cons :ns (lib:sv-ref view 0))
            ,(lib:cons :name (lib:sv-ref view 1))
            ,(lib:cons :value (lib:sv-ref view 2)))))
      (lib:view obj))))

(lib:intern prelude "%inspect-const"
   (:lambda (obj)
     (lib:struct :inspect `(,(prelude:type-of obj) ,(lib:heap-size obj) ()))))

(lib:intern prelude "%inspect-vector"
   (:lambda (obj)
     ((:lambda (tags)          
         (lib:struct :inspect `(vector ,(lib:heap-size obj) ,tags)))
      `(,(lib:cons :type (lib:sv-type obj))
        ,(lib:cons :length (lib:sv-len obj))))))

(lib:intern prelude "%inspect-cons"
   (:lambda (obj)
     ((:lambda (tags)          
         (lib:struct :inspect `(cons ,(lib:heap-size obj) ,tags)))
      `(,(lib:cons :car (lib:car obj))
        ,(lib:cons :cdr (lib:cdr obj))))))

(lib:intern prelude "%inspect"
   (:lambda (value)
      (lib:fix
       (:lambda (loop)
          (:if (prelude:structp loop)
               loop
               (:if (prelude:null loop)
                    (lib:struct :inspect `(unknown 0 ()))
                    ((:lambda (predicate fn)
                       (:if (lib:apply predicate `(,value))
                            (lib:apply fn `(,value))
                            (lib:cdr loop)))
                     (lib:car (lib:car loop))
                     (lib:cdr (lib:car loop))))))
       `(,(lib:cons prelude:%prelude-type-p prelude:%inspect-type)
         ,(lib:cons prelude:charp prelude:%inspect-const)
         ,(lib:cons prelude:consp prelude:%inspect-cons)
         ,(lib:cons prelude:fixnump prelude:%inspect-const)
         ,(lib:cons prelude:floatp prelude:%inspect-const)
         ,(lib:cons prelude:functionp prelude:%inspect-function)
         ,(lib:cons prelude:structp prelude:%inspect-struct)
         ,(lib:cons prelude:symbolp prelude:%inspect-symbol)
         ,(lib:cons prelude:vectorp prelude:%inspect-vector)))))
