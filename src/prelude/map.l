;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; folds and maps
;;;
(libcore:intern :prelude "%foldl"
  (:lambda (function init list)
     (libcore:cdr
      (libcore:fix
       (:lambda (arg)
          ((:lambda (list acc)
             (:if list
                  `(,(libcore:cdr list)
                     ,@(libcore:apply function `(,(libcore:car list) ,acc)))
                   arg))
           (libcore:car arg)
           (libcore:cdr arg)))
       `(,list ,@init)))))

(libcore:intern :prelude "foldl"
  (:lambda (function init list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (libcore:cdr
                (libcore:fix
                 (:lambda (arg)
                    ((:lambda (list acc)
                        (:if list
                             (libcore:cons
                              (libcore:cdr list)
                              (prelude:apply function `(,(libcore:car list) ,acc)))
                             arg))
                     (libcore:car arg)
                     (libcore:cdr arg)))
                 `(,list ,@init)))
               (prelude:raise list 'prelude:foldl "not a list"))
          (prelude:raise function 'prelude:foldl "not a function"))))

(libcore:intern :prelude "foldr"
   (:lambda (function init list)
    (:if (prelude:functionp function)
         (:if (prelude:listp list)
              (prelude:foldl function init (prelude:reverse list))
              (prelude:raise list 'prelude:foldr "not a list"))
         (prelude:raise function 'prelude:foldr "not a function"))))

;;;
;;; maps
;;;
(libcore:intern :prelude "%mapc"
   (:lambda (function list)
     (libcore:fix
      (:lambda (list-cdr)
        (:if list-cdr
             ((:lambda ()
                 (libcore:apply function `(,(libcore:car list-cdr)))
                 (libcore:cdr list-cdr)))
             ()))
      list)
     list))

(libcore:intern :prelude "mapc"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (prelude:%mapc function list)
               (prelude:raise list 'prelude:mapc "not a list"))
          (prelude:raise function 'prelude:mapc "not a function"))
     list))

(libcore:intern :prelude "%mapcar"
   (:lambda (function list)
      (prelude:%foldl
       (:lambda (elt acc)
          `(,@acc ,(libcore:apply function `(,elt))))
       ()
       list)))

(libcore:intern :prelude "mapcar"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (prelude:%mapcar function list)
               (prelude:raise list 'prelude:mapcar "not a list"))
          (prelude:raise function 'prelude:mapcar "not a function"))))

(libcore:intern :prelude "mapl"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (libcore:fix
                 (:lambda (list)
                    (:if list
                         ((:lambda ()
                            (prelude:apply function `(,list))
                            (libcore:cdr list)))
                         ()))
                 list)
                (prelude:raise list 'prelude:mapl "not a list"))
           (prelude:raise function 'prelude:mapl "not a function"))
      list))

(libcore:intern :prelude "maplist"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (libcore:car
                 (libcore:fix
                  (:lambda (loop)
                     ((:lambda (acc list)
                         (:if list
                              (libcore:cons
                               `(,@acc ,(prelude:apply function `(,list)))
                                (libcore:cdr list))
                              loop))
                      (libcore:car loop)
                      (libcore:cdr loop)))
                  `(() ,@list)))
                (prelude:raise list 'prelude:maplist "not a list"))
           (prelude:raise function 'prelude:maplist "not a function"))))

(libcore:intern :prelude "%assoc"
   (:lambda (item alist)
      (libcore:car
       (libcore:fix
        (:lambda (lst)
           (:if lst
                ((:lambda (entry cdr)
                    (:if entry
                         (:if (libcore:eq item (libcore:car entry))
                              lst
                              cdr)
                         cdr))
                 (libcore:car lst)
                 (libcore:cdr lst))
                ()))
        alist))))

(libcore:intern :prelude "assoc"
   (:lambda (item alist)
     (:if (prelude:listp alist)
          (prelude:%assoc item alist)
          (prelude:raise alist 'prelude:assoc "not an alist"))))
