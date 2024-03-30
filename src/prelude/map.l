;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; folds and maps
;;;
(lib:intern :prelude "%foldl"
  (:lambda (function init list)
     (lib:cdr
      (lib:fix
       (:lambda (arg)
          ((:lambda (list acc)
             (:if list
                  `(,(lib:cdr list)
                     ,@(lib:apply function `(,(lib:car list) ,acc)))
                   arg))
           (lib:car arg)
           (lib:cdr arg)))
       `(,list ,@init)))))

(lib:intern :prelude "foldl"
  (:lambda (function init list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (lib:cdr
                (lib:fix
                 (:lambda (arg)
                    ((:lambda (list acc)
                        (:if list
                             (lib:cons
                              (lib:cdr list)
                              (prelude:apply function `(,(lib:car list) ,acc)))
                             arg))
                     (lib:car arg)
                     (lib:cdr arg)))
                 `(,list ,@init)))
               (prelude:raise list 'prelude:foldl "not a list"))
          (prelude:raise function 'prelude:foldl "not a function"))))

(lib:intern :prelude "foldr"
   (:lambda (function init list)
    (:if (prelude:functionp function)
         (:if (prelude:listp list)
              (prelude:foldl function init (prelude:reverse list))
              (prelude:raise list 'prelude:foldr "not a list"))
         (prelude:raise function 'prelude:foldr "not a function"))))

;;;
;;; maps
;;;
(lib:intern :prelude "%mapc"
   (:lambda (function list)
     (lib:fix
      (:lambda (list-cdr)
        (:if list-cdr
             ((:lambda ()
                 (lib:apply function `(,(lib:car list-cdr)))
                 (lib:cdr list-cdr)))
             ()))
      list)
     list))

(lib:intern :prelude "mapc"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (prelude:%mapc function list)
               (prelude:raise list 'prelude:mapc "not a list"))
          (prelude:raise function 'prelude:mapc "not a function"))
     list))

(lib:intern :prelude "%mapcar"
   (:lambda (function list)
      (prelude:%foldl
       (:lambda (elt acc)
          `(,@acc ,(lib:apply function `(,elt))))
       ()
       list)))

(lib:intern :prelude "mapcar"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (prelude:%mapcar function list)
               (prelude:raise list 'prelude:mapcar "not a list"))
          (prelude:raise function 'prelude:mapcar "not a function"))))

(lib:intern :prelude "mapl"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (lib:fix
                 (:lambda (list)
                    (:if list
                         ((:lambda ()
                            (prelude:apply function `(,list))
                            (lib:cdr list)))
                         ()))
                 list)
                (prelude:raise list 'prelude:mapl "not a list"))
           (prelude:raise function 'prelude:mapl "not a function"))
      list))

(lib:intern :prelude "maplist"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (lib:car
                 (lib:fix
                  (:lambda (loop)
                     ((:lambda (acc list)
                         (:if list
                              (lib:cons
                               `(,@acc ,(prelude:apply function `(,list)))
                                (lib:cdr list))
                              loop))
                      (lib:car loop)
                      (lib:cdr loop)))
                  `(() ,@list)))
                (prelude:raise list 'prelude:maplist "not a list"))
           (prelude:raise function 'prelude:maplist "not a function"))))

(lib:intern :prelude "%assoc"
   (:lambda (item alist)
      (lib:car
       (lib:fix
        (:lambda (lst)
           (:if lst
                ((:lambda (entry cdr)
                    (:if entry
                         (:if (lib:eq item (lib:car entry))
                              lst
                              cdr)
                         cdr))
                 (lib:car lst)
                 (lib:cdr lst))
                ()))
        alist))))

(lib:intern :prelude "assoc"
   (:lambda (item alist)
     (:if (prelude:listp alist)
          (prelude:%assoc item alist)
          (prelude:raise alist 'prelude:assoc "not an alist"))))
