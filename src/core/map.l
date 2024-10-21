;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; folds
;;;
(mu:intern core "%foldl"
  (:lambda (function init list)
     (mu:cdr
      (mu:fix
       (:lambda (arg)
          ((:lambda (list acc)
             (:if list
                  (mu:cons
                   (mu:cdr list)
                   (core:%apply function (mu:cons (mu:car list) (mu:cons acc ()))))
                  arg))
           (mu:car arg)
           (mu:cdr arg)))
       (mu:cons list init)))))

(mu:intern core "%foldr"
   (:lambda (function init list)
     (core:%foldl function init (core:%reverse list))))

;;;
;;; maps
;;;
(mu:intern core "%mapc"
   (:lambda (function list)
     (mu:fix
      (:lambda (list-cdr)
        (:if list-cdr
             (core:%arg2
                 (core:apply function (mu:cons (mu:car list-cdr) ()))
                 (mu:cdr list-cdr))
             ()))
      list)
     list))

(mu:intern core "%mapcar"
   (:lambda (function list)
      (core:%foldl
       (:lambda (elt acc)
          `(,@acc ,(core:apply function `(,elt))))
       ()
       list)))

(mu:intern core "%mapl"
   (:lambda (function list)
     (mu:fix
      (:lambda (list)
        (:if list
             (core:%arg2
                (core:%apply function (mu:cons list ()))
                (mu:cdr list))
             ()))
      list)
     list))

(mu:intern core "%maplist"
   (:lambda (function list)
     (mu:car
      (mu:fix
       (:lambda (loop)
         ((:lambda (acc list)
            (:if list
                 (mu:cons
                  `(,@acc ,(core:%apply function `(,list)))
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
        (:lambda (lst)
           (:if lst
                ((:lambda (entry cdr)
                    (:if entry
                         (:if (mu:eq item (mu:car entry))
                              lst
                              cdr)
                         cdr))
                 (mu:car lst)
                 (mu:cdr lst))
                ()))
        alist))))

(mu:intern core "%rassq"
   (:lambda (item alist)
      (mu:car
       (mu:fix
        (:lambda (lst)
           (:if lst
                ((:lambda (entry cdr)
                    (:if entry
                         (:if (mu:eq item (mu:cdr entry))
                              lst
                              cdr)
                         cdr))
                 (mu:car lst)
                 (mu:cdr lst))
                ()))
        alist))))
