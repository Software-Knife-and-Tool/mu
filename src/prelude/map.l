;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; folds and maps
;;;
(crux:intern prelude "%foldl"
  (:lambda (function init list)
     (crux:cdr
      (crux:fix
       (:lambda (arg)
          ((:lambda (list acc)
             (:if list
                  `(,(crux:cdr list)
                     ,@(crux:apply function `(,(crux:car list) ,acc)))
                   arg))
           (crux:car arg)
           (crux:cdr arg)))
       `(,list ,@init)))))

(crux:intern prelude "foldl"
  (:lambda (function init list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (crux:cdr
                (crux:fix
                 (:lambda (arg)
                    ((:lambda (list acc)
                        (:if list
                             (crux:cons
                              (crux:cdr list)
                              (prelude:apply function `(,(crux:car list) ,acc)))
                             arg))
                     (crux:car arg)
                     (crux:cdr arg)))
                 `(,list ,@init)))
               (prelude:raise list 'prelude:foldl "not a list"))
          (prelude:raise function 'prelude:foldl "not a function"))))

(crux:intern prelude "foldr"
   (:lambda (function init list)
    (:if (prelude:functionp function)
         (:if (prelude:listp list)
              (prelude:foldl function init (prelude:reverse list))
              (prelude:raise list 'prelude:foldr "not a list"))
         (prelude:raise function 'prelude:foldr "not a function"))))

;;;
;;; maps
;;;
(crux:intern prelude "%mapc"
   (:lambda (function list)
     (crux:fix
      (:lambda (list-cdr)
        (:if list-cdr
             ((:lambda ()
                 (crux:apply function `(,(crux:car list-cdr)))
                 (crux:cdr list-cdr)))
             ()))
      list)
     list))

(crux:intern prelude "mapc"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (prelude:%mapc function list)
               (prelude:raise list 'prelude:mapc "not a list"))
          (prelude:raise function 'prelude:mapc "not a function"))
     list))

(crux:intern prelude "%mapcar"
   (:lambda (function list)
      (prelude:%foldl
       (:lambda (elt acc)
          `(,@acc ,(crux:apply function `(,elt))))
       ()
       list)))

(crux:intern prelude "mapcar"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (prelude:%mapcar function list)
               (prelude:raise list 'prelude:mapcar "not a list"))
          (prelude:raise function 'prelude:mapcar "not a function"))))

(crux:intern prelude "mapl"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (crux:fix
                 (:lambda (list)
                    (:if list
                         ((:lambda ()
                            (prelude:apply function `(,list))
                            (crux:cdr list)))
                         ()))
                 list)
                (prelude:raise list 'prelude:mapl "not a list"))
           (prelude:raise function 'prelude:mapl "not a function"))
      list))

(crux:intern prelude "maplist"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (crux:car
                 (crux:fix
                  (:lambda (loop)
                     ((:lambda (acc list)
                         (:if list
                              (crux:cons
                               `(,@acc ,(prelude:apply function `(,list)))
                                (crux:cdr list))
                              loop))
                      (crux:car loop)
                      (crux:cdr loop)))
                  `(() ,@list)))
                (prelude:raise list 'prelude:maplist "not a list"))
           (prelude:raise function 'prelude:maplist "not a function"))))

(crux:intern prelude "%assq"
   (:lambda (item alist)
      (crux:car
       (crux:fix
        (:lambda (lst)
           (:if lst
                ((:lambda (entry cdr)
                    (:if entry
                         (:if (crux:eq item (crux:car entry))
                              lst
                              cdr)
                         cdr))
                 (crux:car lst)
                 (crux:cdr lst))
                ()))
        alist))))

(crux:intern prelude "assq"
   (:lambda (item alist)
     (:if (prelude:listp alist)
          (prelude:%assq item alist)
          (prelude:raise alist 'prelude:assq "not an alist"))))

(crux:intern prelude "%rassq"
   (:lambda (item alist)
      (crux:car
       (crux:fix
        (:lambda (lst)
           (:if lst
                ((:lambda (entry cdr)
                    (:if entry
                         (:if (crux:eq item (crux:cdr entry))
                              lst
                              cdr)
                         cdr))
                 (crux:car lst)
                 (crux:cdr lst))
                ()))
        alist))))

(crux:intern prelude "rassq"
   (:lambda (item alist)
     (:if (prelude:listp alist)
          (prelude:%rassq item alist)
          (prelude:raise alist 'prelude:assq "not an alist"))))
