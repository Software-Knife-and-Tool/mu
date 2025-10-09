;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;

;;;
;;; folds
;;;
(mu:intern core "%foldl"
  (:lambda (func init list)
     (mu:cdr
      (mu:fix
       (:lambda (arg)
          ((:lambda (list acc)
             (:if list
                  (mu:cons
                   (mu:cdr list)
                   (core:apply func (core:%list2 (mu:car list) acc)))
                  arg))
           (mu:car arg)
           (mu:cdr arg)))
       (mu:cons list init)))))

(mu:intern core "%foldr"
   (:lambda (func init list)
     (mu:cdr
      (mu:fix
       (:lambda (arg)
          ((:lambda (nth acc)
             (:if (mu:less-than nth 0)
                  arg
                  (mu:cons
                   nth
                   (core:apply func (core:%list2 (mu:nth nth list) acc)))))
           (mu:sub (mu:car arg) 1)
           (mu:cdr arg)))
       (mu:cons (mu:length list) init)))))

;;;
;;; maps
;;;
(mu:intern core "%mapc"
   (:lambda (func list)
     (mu:fix
      (:lambda (list-cdr)
        (:if list-cdr
             (core:%prog2
                 (core:apply func (mu:cons (mu:car list-cdr) ()))
                 (mu:cdr list-cdr))
             ()))
      list)
     list))

(mu:intern core "%mapcar"
   (:lambda (func list)
      (core:%foldl
       (:lambda (elt acc)
          `(,@acc ,(core:apply func `(,elt))))
       ()
       list)))

(mu:intern core "%mapl"
   (:lambda (func list)
     (mu:fix
      (:lambda (list)
        (:if list
             (core:%prog2
                (core:apply func (mu:cons list ()))
                (mu:cdr list))
             ()))
      list)
     list))

(mu:intern core "%maplist"
   (:lambda (func list)
     (mu:car
      (mu:fix
       (:lambda (loop)
         ((:lambda (acc list)
            (:if list
                 (mu:cons
                  `(,@acc ,(core:apply func `(,list)))
                  (mu:cdr list))
                 loop))
          (mu:car loop)
          (mu:cdr loop)))
       (mu:cons () list)))))

;;;
;;; association lists
;;;
(mu:intern core "%assq"
   (:lambda (item alist)
      (mu:car
       (mu:fix
        (:lambda (list)
           (:if list
                ((:lambda (entry cdr)
                    (:if entry
                         (:if (mu:eq item (mu:car entry))
                              list
                              cdr)
                         cdr))
                 (mu:car list)
                 (mu:cdr list))
                ()))
        alist))))

(mu:intern core "%rassq"
   (:lambda (item alist)
      (mu:car
       (mu:fix
        (:lambda (list)
           (:if list
                ((:lambda (entry cdr)
                    (:if entry
                         (:if (mu:eq item (mu:cdr entry))
                              list
                              cdr)
                         cdr))
                 (mu:car list)
                 (mu:cdr list))
                ()))
        alist))))

;;;
;;; finds
;;;
(mu:intern core "%find-if"
   (:lambda (predicate list)
     (mu:cdr
      (mu:fix
       (:lambda (loop)
         (:if (mu:eq :t (mu:car loop))
              loop
              (:if (mu:cdr loop)
                   ((:lambda (el)
                      (:if (core:apply predicate (mu:cons el ()))
                           (mu:cons :t el)
                           (mu:cons () (mu:nthcdr 2 loop))))
                    (mu:nth 1 loop))
                   loop)))
       (mu:cons () list)))))

;;;
;;; positions
;;;
(mu:intern core "%position-if"
   (:lambda (func list)
     (:if list
          ((:lambda (length)
             (mu:car
              (mu:fix
               (:lambda (loop)
                 (:if (core:listp loop)
                      loop
                      (:if (mu:less-than length loop)
                           ()
                           (:if (core:apply func (mu:cons (mu:nth loop list) ()))
                                (mu:cons loop  ())
                                (mu:add loop 1)))))
               0)))
           (mu:sub (mu:length list) 1))
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
