;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;

;;;
;;; finds
;;;
(mu:intern core "%findl"
   (:lambda (function list)
     (mu:cdr
      (mu:fix
       (:lambda (loop)
         (:if (mu:eq :t (mu:car loop))
              loop
              (:if (mu:cdr loop)
                   ((:lambda (el)
                      (:if (core:%apply function (mu:cons el ()))
                           (mu:cons :t el)
                           (mu:cons () (mu:nthcdr 2 loop))))
                    (mu:nth 1 loop))
                   loop)))
       (mu:cons () list)))))

;;;
;;; positions
;;;
(mu:intern core "%positionl"
   (:lambda (function list)
     (:if list
          ((:lambda (length)
             (mu:car
              (mu:fix
               (:lambda (loop)
                 (:if (core:listp loop)
                      loop
                      (:if (mu:less-than length loop)
                           ()
                           (:if (core:%apply function (mu:cons (mu:nth loop list) ()))
                                (mu:cons loop  ())
                                (mu:add loop 1)))))
               0)))
           (mu:length list))
          ())))
   
;;;
;;; fixed argument list functions
;;;
(mu:intern core "%append"
   (:lambda (list)
     (core:%foldr
      (:lambda (el acc)
        (:if (core:listp el)
             (core:%foldr mu:cons acc el)
             el))
      ()
      list)))

(mu:intern core "%reverse"
   (:lambda (list)
     (core:%foldl mu:cons () list)))

(mu:intern core "%dropl"
   (:lambda (list n)
     (mu:nthcdr n list)))

(mu:intern core "%dropr"
   (:lambda (list n)
     (core:%foldl
      mu:cons
      ()
      ((:lambda (length)
         (core:%foldl
          (:lambda (el last)
            (:if (mu:less-than (mu:length last) (mu:sub length n))
                 (mu:cons el last)
                 last))
          ()
          list))
       (mu:length list)))))
