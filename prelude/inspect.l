;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; inspect
;;;
(mu:intern :prelude "%inspect-function"
   (:lambda (obj)
      ((:lambda (view)
          ((:lambda (tags)
             (mu:struct :inspect `(function ,(mu:hp-size obj) ,tags)))
           `(,(mu:cons :arity (mu:sv-ref view 0))
             ,(mu:cons :body (mu:sv-ref view 1)))))
       (mu:view obj))))

(mu:intern :prelude "%inspect-type"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)
            (mu:struct :inspect `(:<type> ,(mu:hp-size obj) ,tags)))
          `(,(mu:cons :type (mu:sv-ref (mu:st-vec obj) 0))
            ,(mu:cons :values (mu:sv-ref (mu:st-vec obj) 1)))))
      (mu:view obj))))

(mu:intern :prelude "%inspect-struct"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (mu:struct :inspect `(struct ,(mu:hp-size obj) ,tags)))
          `(,(mu:cons :type (mu:st-type obj))
            ,(mu:cons :values (mu:st-vec obj)))))
      (mu:view obj))))

(mu:intern :prelude "%inspect-symbol"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (mu:struct :inspect `(symbol ,(mu:hp-size obj) ,tags)))
          `(,(mu:cons :ns (mu:sv-ref view 0))
            ,(mu:cons :name (mu:sv-ref view 1))
            ,(mu:cons :value (mu:sv-ref view 2)))))
      (mu:view obj))))

(mu:intern :prelude "%inspect-const"
   (:lambda (obj)
     (mu:struct :inspect `(,(prelude:type-of obj) ,(mu:hp-size obj) ()))))

(mu:intern :prelude "%inspect-vector"
   (:lambda (obj)
     ((:lambda (tags)          
         (mu:struct :inspect `(vector ,(mu:hp-size obj) ,tags)))
      `(,(mu:cons :type (mu:sv-type obj))
        ,(mu:cons :length (mu:sv-len obj))))))

(mu:intern :prelude "%inspect-cons"
   (:lambda (obj)
     ((:lambda (tags)          
         (mu:struct :inspect `(cons ,(mu:hp-size obj) ,tags)))
      `(,(mu:cons :car (mu:car obj))
        ,(mu:cons :cdr (mu:cdr obj))))))

(mu:intern :prelude "%inspect"
   (:lambda (value)
      (mu:fix
       (:lambda (loop)
          (:if (prelude:structp loop)
               loop
               (:if (prelude:null loop)
                    (mu:struct :inspect `(unknown 0 ()))
                    ((:lambda (predicate fn)
                       (:if (mu:apply predicate `(,value))
                            (mu:apply fn `(,value))
                            (mu:cdr loop)))
                     (mu:car (mu:car loop))
                     (mu:cdr (mu:car loop))))))
       `(,(mu:cons prelude:%prelude-type-p prelude:%inspect-type)
         ,(mu:cons prelude:charp prelude:%inspect-const)
         ,(mu:cons prelude:consp prelude:%inspect-cons)
         ,(mu:cons prelude:fixnump prelude:%inspect-const)
         ,(mu:cons prelude:floatp prelude:%inspect-const)
         ,(mu:cons prelude:functionp prelude:%inspect-function)
         ,(mu:cons prelude:structp prelude:%inspect-struct)
         ,(mu:cons prelude:symbolp prelude:%inspect-symbol)
         ,(mu:cons prelude:vectorp prelude:%inspect-vector)))))
