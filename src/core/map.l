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

(mu:intern core "foldl"
  (:lambda (function init list)
     (:if (core:functionp function)
          (:if (core:listp list)
               (core:%foldl function init list)
               (core:raise list 'core:foldl "not a list"))
          (core:raise function 'core:foldl "not a function"))))

(mu:intern core "foldr"
   (:lambda (function init list)
    (:if (core:functionp function)
         (:if (core:listp list)
              (core:foldl function init (core:reverse list))
              (core:raise list 'core:foldr "not a list"))
         (core:raise function 'core:foldr "not a function"))))

;;;
;;; maps
;;;
(mu:intern core "%mapc"
   (:lambda (function list)
     (mu:fix
      (:lambda (list-cdr)
        (:if list-cdr
             ((:lambda ()
                 (mu:apply function (mu:cons (mu:car list-cdr) ()))
                 (mu:cdr list-cdr)))
             ()))
      list)
     list))

(mu:intern core "mapc"
   (:lambda (function list)
     (:if (core:functionp function)
          (:if (core:listp list)
               (core:%mapc function list)
               (core:raise list 'core:mapc "not a list"))
          (core:raise function 'core:mapc "not a function"))
     list))

(mu:intern core "%mapcar"
   (:lambda (function list)
      (core:%foldl
       (:lambda (elt acc)
          `(,@acc ,(mu:apply function `(,elt))))
       ()
       list)))

(mu:intern core "mapcar"
   (:lambda (function list)
     (:if (core:functionp function)
          (:if (core:listp list)
               (core:%mapcar function list)
               (core:raise list 'core:mapcar "not a list"))
          (core:raise function 'core:mapcar "not a function"))))

(mu:intern core "mapl"
   (:lambda (function list)
      (:if (core:functionp function)
           (:if (core:listp list)
                (mu:fix
                 (:lambda (list)
                    (:if list
                         ((:lambda ()
                            (core:%apply function `(,list))
                            (mu:cdr list)))
                         ()))
                 list)
                (core:raise list 'core:mapl "not a list"))
           (core:raise function 'core:mapl "not a function"))
      list))

(mu:intern core "maplist"
   (:lambda (function list)
      (:if (core:functionp function)
           (:if (core:listp list)
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
                  `(() ,@list)))
                (core:raise list 'core:maplist "not a list"))
           (core:raise function 'core:maplist "not a function"))))

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

(mu:intern core "assq"
   (:lambda (item alist)
     (:if (core:listp alist)
          (core:%assq item alist)
          (core:raise alist 'core:assq "not an alist"))))

(mu:intern core "rassq"
   (:lambda (item alist)
     (:if (core:listp alist)
          (core:%rassq item alist)
          (core:raise alist 'core:rassq "not an alist"))))
