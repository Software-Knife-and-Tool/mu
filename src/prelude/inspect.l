;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; inspect
;;;
(libcore:intern :prelude "%inspect-function"
   (:lambda (obj)
      ((:lambda (view)
          ((:lambda (tags)
             (libcore:struct :inspect `(function ,(libcore:hp-size obj) ,tags)))
           `(,(libcore:cons :arity (libcore:sv-ref view 0))
             ,(libcore:cons :body (libcore:sv-ref view 1)))))
       (libcore:view obj))))

(libcore:intern :prelude "%inspect-type"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)
            (libcore:struct :inspect `(:<type> ,(libcore:hp-size obj) ,tags)))
          `(,(libcore:cons :type (libcore:sv-ref (libcore:st-vec obj) 0))
            ,(libcore:cons :values (libcore:sv-ref (libcore:st-vec obj) 1)))))
      (libcore:view obj))))

(libcore:intern :prelude "%inspect-struct"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (libcore:struct :inspect `(struct ,(libcore:hp-size obj) ,tags)))
          `(,(libcore:cons :type (libcore:st-type obj))
            ,(libcore:cons :values (libcore:st-vec obj)))))
      (libcore:view obj))))

(libcore:intern :prelude "%inspect-symbol"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (libcore:struct :inspect `(symbol ,(libcore:hp-size obj) ,tags)))
          `(,(libcore:cons :ns (libcore:sv-ref view 0))
            ,(libcore:cons :name (libcore:sv-ref view 1))
            ,(libcore:cons :value (libcore:sv-ref view 2)))))
      (libcore:view obj))))

(libcore:intern :prelude "%inspect-const"
   (:lambda (obj)
     (libcore:struct :inspect `(,(prelude:type-of obj) ,(libcore:hp-size obj) ()))))

(libcore:intern :prelude "%inspect-vector"
   (:lambda (obj)
     ((:lambda (tags)          
         (libcore:struct :inspect `(vector ,(libcore:hp-size obj) ,tags)))
      `(,(libcore:cons :type (libcore:sv-type obj))
        ,(libcore:cons :length (libcore:sv-len obj))))))

(libcore:intern :prelude "%inspect-cons"
   (:lambda (obj)
     ((:lambda (tags)          
         (libcore:struct :inspect `(cons ,(libcore:hp-size obj) ,tags)))
      `(,(libcore:cons :car (libcore:car obj))
        ,(libcore:cons :cdr (libcore:cdr obj))))))

(libcore:intern :prelude "%inspect"
   (:lambda (value)
      (libcore:fix
       (:lambda (loop)
          (:if (prelude:structp loop)
               loop
               (:if (prelude:null loop)
                    (libcore:struct :inspect `(unknown 0 ()))
                    ((:lambda (predicate fn)
                       (:if (libcore:apply predicate `(,value))
                            (libcore:apply fn `(,value))
                            (libcore:cdr loop)))
                     (libcore:car (libcore:car loop))
                     (libcore:cdr (libcore:car loop))))))
       `(,(libcore:cons prelude:%prelude-type-p prelude:%inspect-type)
         ,(libcore:cons prelude:charp prelude:%inspect-const)
         ,(libcore:cons prelude:consp prelude:%inspect-cons)
         ,(libcore:cons prelude:fixnump prelude:%inspect-const)
         ,(libcore:cons prelude:floatp prelude:%inspect-const)
         ,(libcore:cons prelude:functionp prelude:%inspect-function)
         ,(libcore:cons prelude:structp prelude:%inspect-struct)
         ,(libcore:cons prelude:symbolp prelude:%inspect-symbol)
         ,(libcore:cons prelude:vectorp prelude:%inspect-vector)))))
