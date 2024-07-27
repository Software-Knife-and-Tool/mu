;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; inspect
;;;
(mu:intern prelude "%inspect-function"
   (:lambda (obj)
      ((:lambda (view)
          ((:lambda (tags)
             (mu:make-struct :inspect `(function ,(mu:heap-size obj) ,tags)))
           `(,(mu:cons :arity (mu:svref view 0))
             ,(mu:cons :body (mu:svref view 1)))))
       (mu:view obj))))

(mu:intern prelude "%inspect-type"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)
            (mu:make-struct :inspect `(:<type> ,(mu:heap-size obj) ,tags)))
          `(,(mu:cons :type (mu:svref (mu:struct-vec obj) 0))
            ,(mu:cons :values (mu:svref (mu:struct-vec obj) 1)))))
      (mu:view obj))))

(mu:intern prelude "%inspect-struct"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (mu:make-struct :inspect `(struct ,(mu:heap-size obj) ,tags)))
          `(,(mu:cons :type (mu:struct-type obj))
            ,(mu:cons :values (mu:struct-vec obj)))))
      (mu:view obj))))

(mu:intern prelude "%inspect-symbol"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (mu:make-struct :inspect `(symbol ,(mu:heap-size obj) ,tags)))
          `(,(mu:cons :ns (mu:svref view 0))
            ,(mu:cons :name (mu:svref view 1))
            ,(mu:cons :value (mu:svref view 2)))))
      (mu:view obj))))

(mu:intern prelude "%inspect-const"
   (:lambda (obj)
     (mu:make-struct :inspect `(,(prelude:type-of obj) ,(mu:heap-size obj) ()))))

(mu:intern prelude "%inspect-vector"
   (:lambda (obj)
     ((:lambda (tags)          
         (mu:make-struct :inspect `(vector ,(mu:heap-size obj) ,tags)))
      `(,(mu:cons :type (mu:vector-type obj))
        ,(mu:cons :length (mu:vector-len obj))))))

(mu:intern prelude "%inspect-cons"
   (:lambda (obj)
     ((:lambda (tags)          
         (mu:make-struct :inspect `(cons ,(mu:heap-size obj) ,tags)))
      `(,(mu:cons :car (mu:car obj))
        ,(mu:cons :cdr (mu:cdr obj))))))

(mu:intern prelude "%inspect"
   (:lambda (value)
      (mu:fix
       (:lambda (loop)
          (:if (prelude:structp loop)
               loop
               (:if (prelude:null loop)
                    (mu:make-struct :inspect `(unknown 0 ()))
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
