;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; folds and maps
;;;
(mu:intern :prelude "%foldl"
  (:lambda (function init list)
     (mu:cdr
      (mu:fix
       (:lambda (arg)
          ((:lambda (list acc)
             (:if list
                  `(,(mu:cdr list)
                     ,@(mu:apply function `(,(mu:car list) ,acc)))
                   arg))
           (mu:car arg)
           (mu:cdr arg)))
       `(,list ,@init)))))

(mu:intern :prelude "foldl"
  (:lambda (function init list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (mu:cdr
                (mu:fix
                 (:lambda (arg)
                    ((:lambda (list acc)
                        (:if list
                             (mu:cons
                              (mu:cdr list)
                              (prelude:apply function `(,(mu:car list) ,acc)))
                             arg))
                     (mu:car arg)
                     (mu:cdr arg)))
                 `(,list ,@init)))
               (prelude:raise list 'prelude:foldl "not a list"))
          (prelude:raise function 'prelude:foldl "not a function"))))

(mu:intern :prelude "foldr"
   (:lambda (function init list)
    (:if (prelude:functionp function)
         (:if (prelude:listp list)
              (prelude:foldl function init (prelude:reverse list))
              (prelude:raise list 'prelude:foldr "not a list"))
         (prelude:raise function 'prelude:foldr "not a function"))))

;;;
;;; maps
;;;
(mu:intern :prelude "%mapc"
   (:lambda (function list)
     (mu:fix
      (:lambda (list-cdr)
        (:if list-cdr
             ((:lambda ()
                 (mu:apply function `(,(mu:car list-cdr)))
                 (mu:cdr list-cdr)))
             ()))
      list)
     list))

(mu:intern :prelude "mapc"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (prelude:%mapc function list)
               (prelude:raise list 'prelude:mapc "not a list"))
          (prelude:raise function 'prelude:mapc "not a function"))
     list))

(mu:intern :prelude "%mapcar"
   (:lambda (function list)
      (prelude:%foldl
       (:lambda (elt acc)
          `(,@acc ,(mu:apply function `(,elt))))
       ()
       list)))

(mu:intern :prelude "mapcar"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (prelude:%mapcar function list)
               (prelude:raise list 'prelude:mapcar "not a list"))
          (prelude:raise function 'prelude:mapcar "not a function"))))

(mu:intern :prelude "mapl"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (mu:fix
                 (:lambda (list)
                    (:if list
                         ((:lambda ()
                            (prelude:apply function `(,list))
                            (mu:cdr list)))
                         ()))
                 list)
                (prelude:raise list 'prelude:mapl "not a list"))
           (prelude:raise function 'prelude:mapl "not a function"))
      list))

(mu:intern :prelude "maplist"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (mu:car
                 (mu:fix
                  (:lambda (loop)
                     ((:lambda (acc list)
                         (:if list
                              (mu:cons
                               `(,@acc ,(prelude:apply function `(,list)))
                                (mu:cdr list))
                              loop))
                      (mu:car loop)
                      (mu:cdr loop)))
                  `(() ,@list)))
                (prelude:raise list 'prelude:maplist "not a list"))
           (prelude:raise function 'prelude:maplist "not a function"))))

(mu:intern :prelude "%assoc"
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

(mu:intern :prelude "assoc"
   (:lambda (item alist)
     (:if (prelude:listp alist)
          (prelude:%assoc item alist)
          (prelude:raise alist 'prelude:assoc "not an alist"))))
