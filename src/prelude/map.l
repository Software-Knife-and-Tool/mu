;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; folds and maps
;;;
(core:intern :prelude "%foldl"
  (:lambda (function init list)
     (core:cdr
      (core:fix
       (:lambda (arg)
          ((:lambda (list acc)
             (:if list
                  `(,(core:cdr list)
                     ,@(core:apply function `(,(core:car list) ,acc)))
                   arg))
           (core:car arg)
           (core:cdr arg)))
       `(,list ,@init)))))

(core:intern :prelude "foldl"
  (:lambda (function init list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (core:cdr
                (core:fix
                 (:lambda (arg)
                    ((:lambda (list acc)
                        (:if list
                             (core:cons
                              (core:cdr list)
                              (prelude:apply function `(,(core:car list) ,acc)))
                             arg))
                     (core:car arg)
                     (core:cdr arg)))
                 `(,list ,@init)))
               (prelude:raise list 'prelude:foldl "not a list"))
          (prelude:raise function 'prelude:foldl "not a function"))))

(core:intern :prelude "foldr"
   (:lambda (function init list)
    (:if (prelude:functionp function)
         (:if (prelude:listp list)
              (prelude:foldl function init (prelude:reverse list))
              (prelude:raise list 'prelude:foldr "not a list"))
         (prelude:raise function 'prelude:foldr "not a function"))))

;;;
;;; maps
;;;
(core:intern :prelude "%mapc"
   (:lambda (function list)
     (core:fix
      (:lambda (list-cdr)
        (:if list-cdr
             ((:lambda ()
                 (core:apply function `(,(core:car list-cdr)))
                 (core:cdr list-cdr)))
             ()))
      list)
     list))

(core:intern :prelude "mapc"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (prelude:%mapc function list)
               (prelude:raise list 'prelude:mapc "not a list"))
          (prelude:raise function 'prelude:mapc "not a function"))
     list))

(core:intern :prelude "%mapcar"
   (:lambda (function list)
      (prelude:%foldl
       (:lambda (elt acc)
          `(,@acc ,(core:apply function `(,elt))))
       ()
       list)))

(core:intern :prelude "mapcar"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (prelude:%mapcar function list)
               (prelude:raise list 'prelude:mapcar "not a list"))
          (prelude:raise function 'prelude:mapcar "not a function"))))

(core:intern :prelude "mapl"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (core:fix
                 (:lambda (list)
                    (:if list
                         ((:lambda ()
                            (prelude:apply function `(,list))
                            (core:cdr list)))
                         ()))
                 list)
                (prelude:raise list 'prelude:mapl "not a list"))
           (prelude:raise function 'prelude:mapl "not a function"))
      list))

(core:intern :prelude "maplist"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (core:car
                 (core:fix
                  (:lambda (loop)
                     ((:lambda (acc list)
                         (:if list
                              (core:cons
                               `(,@acc ,(prelude:apply function `(,list)))
                                (core:cdr list))
                              loop))
                      (core:car loop)
                      (core:cdr loop)))
                  `(() ,@list)))
                (prelude:raise list 'prelude:maplist "not a list"))
           (prelude:raise function 'prelude:maplist "not a function"))))

(core:intern :prelude "%assoc"
   (:lambda (item alist)
      (core:car
       (core:fix
        (:lambda (lst)
           (:if lst
                ((:lambda (entry cdr)
                    (:if entry
                         (:if (core:eq item (core:car entry))
                              lst
                              cdr)
                         cdr))
                 (core:car lst)
                 (core:cdr lst))
                ()))
        alist))))

(core:intern :prelude "assoc"
   (:lambda (item alist)
     (:if (prelude:listp alist)
          (prelude:%assoc item alist)
          (prelude:raise alist 'prelude:assoc "not an alist"))))
