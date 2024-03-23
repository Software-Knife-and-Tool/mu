;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; inspect
;;;
(core:intern :prelude "%inspect-function"
   (:lambda (obj)
      ((:lambda (view)
          ((:lambda (tags)
             (core:struct :inspect `(function ,(core:hp-size obj) ,tags)))
           `(,(core:cons :arity (core:sv-ref view 0))
             ,(core:cons :body (core:sv-ref view 1)))))
       (core:view obj))))

(core:intern :prelude "%inspect-type"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)
            (core:struct :inspect `(:<type> ,(core:hp-size obj) ,tags)))
          `(,(core:cons :type (core:sv-ref (core:st-vec obj) 0))
            ,(core:cons :values (core:sv-ref (core:st-vec obj) 1)))))
      (core:view obj))))

(core:intern :prelude "%inspect-struct"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (core:struct :inspect `(struct ,(core:hp-size obj) ,tags)))
          `(,(core:cons :type (core:st-type obj))
            ,(core:cons :values (core:st-vec obj)))))
      (core:view obj))))

(core:intern :prelude "%inspect-symbol"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (core:struct :inspect `(symbol ,(core:hp-size obj) ,tags)))
          `(,(core:cons :ns (core:sv-ref view 0))
            ,(core:cons :name (core:sv-ref view 1))
            ,(core:cons :value (core:sv-ref view 2)))))
      (core:view obj))))

(core:intern :prelude "%inspect-const"
   (:lambda (obj)
     (core:struct :inspect `(,(prelude:type-of obj) ,(core:hp-size obj) ()))))

(core:intern :prelude "%inspect-vector"
   (:lambda (obj)
     ((:lambda (tags)          
         (core:struct :inspect `(vector ,(core:hp-size obj) ,tags)))
      `(,(core:cons :type (core:sv-type obj))
        ,(core:cons :length (core:sv-len obj))))))

(core:intern :prelude "%inspect-cons"
   (:lambda (obj)
     ((:lambda (tags)          
         (core:struct :inspect `(cons ,(core:hp-size obj) ,tags)))
      `(,(core:cons :car (core:car obj))
        ,(core:cons :cdr (core:cdr obj))))))

(core:intern :prelude "%inspect"
   (:lambda (value)
      (core:fix
       (:lambda (loop)
          (:if (prelude:structp loop)
               loop
               (:if (prelude:null loop)
                    (core:struct :inspect `(unknown 0 ()))
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
