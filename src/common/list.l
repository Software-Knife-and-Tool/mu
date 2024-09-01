;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;

;;;
;;; maps
;;;
(mu:intern common "%mapc"
   (:lambda (function list)
     (mu:fix
      (:lambda (list-cdr)
        (:if list-cdr
             ((:lambda ()
                 (core:%apply function (mu:cons (mu:car list-cdr) ()))
                 (mu:cdr list-cdr)))
             ()))
      list)
     list))

(mu:intern common "mapc"
   (:lambda (function list)
     (:if (core:functionp function)
          (:if (core:listp list)
               (core:%mapc function list)
               (core:raise list 'common:mapc "not a list"))
          (core:raise function 'common:mapc "not a function"))
     list))

(mu:intern common "%mapcar"
   (:lambda (function list)
      (core:%foldl
       (:lambda (elt acc)
          `(,@acc ,(mu:apply function `(,elt))))
       ()
       list)))

(mu:intern common "mapcar"
   (:lambda (function list)
     (:if (core:functionp function)
          (:if (core:listp list)
               (common:%mapcar function list)
               (core:raise list 'common:mapcar "not a list"))
          (core:raise function 'common:mapcar "not a function"))))

(mu:intern common "mapl"
   (:lambda (function list)
      (:if (common:functionp function)
           (:if (core:listp list)
                (mu:fix
                 (:lambda (list)
                    (:if list
                         ((:lambda ()
                            (core:%apply function `(,list))
                            (mu:cdr list)))
                         ()))
                 list)
                (core:raise list 'common:mapl "not a list"))
           (core:raise function 'common:mapl "not a function"))
      list))

(mu:intern common "maplist"
   (:lambda (function list)
      (:if (core:functionp function)
           (:if (core:listp list)
                (mu:car
                 (mu:fix
                  (:lambda (loop)
                     ((:lambda (acc list)
                         (:if list
                              (mu:cons
                               `(,@acc ,(common:%apply function `(,list)))
                                (mu:cdr list))
                              loop))
                      (mu:car loop)
                      (mu:cdr loop)))
                  `(() ,@list)))
                (core:raise list 'common:maplist "not a list"))
           (core:raise function 'common:maplist "not a function"))))

;;;
;;; finds
;;;
(mu:intern common "findl"
   (:lambda (function list)
      (:if (core:functionp function)
           (:if (core:listp list)
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
                  (mu:cons () list)))
                (core:raise list 'common:findl "not a list"))
           (core:raise function 'common:findl "not a function"))))

(mu:intern common "findr"
   (:lambda (function list)
     (common:findl function (common:reverse list))))

;;;
;;; positions
;;;
(mu:intern common "positionl"
   (:lambda (function list)
     (:if (core:functionp function)
          (:if (core:listp list)
               (:if list
                    ((:lambda (length)
                       (mu:car
                        (mu:fix
                         (:lambda (loop)
                           (:if (core:listp loop)
                                loop
                                (:if (mu:less-than length loop)
                                     ()
                                     (:if (common:%apply function (mu:cons (mu:nth loop list) ()))
                                          (mu:cons loop  ())
                                          (common:1+ loop)))))
                         0)))
                     (mu:length list))
                    ())
               (core:raise list 'common:positionl "not a list"))
          (core:raise function 'common:positionl "not a function"))))

(mu:intern common "positionr"
   (:lambda (function list)
      (:if (core:functionp function)
           (:if (core:listp list)
                (:if (core:null list)
                     ()
                     (mu:car
                      (mu:fix
                       (:lambda (loop)
                          (:if (core:listp loop)
                               loop
                               (:if (mu:less-than loop 0)
                                    ()
                                    (:if (core:%apply function (mu:cons (mu:nth loop list) ()))
                                         (mu:cons loop ())
                                         (common:1- loop)))))
                       (mu:sub (mu:length list) 1))))
                (core:raise list 'common:positionr "not a list"))
           (core:raise function 'common:positionr "not a function"))))

(mu:intern common "reverse"
   (:lambda (list)
     (:if (core:listp list)
          (core:%reverse list)
          (core:raise list 'common:reverse "not a list"))))

(mu:intern common "last"
   (:lambda (list)
      (:if (core:listp list)
           (mu:fix
            (:lambda (loop)
               (:if (core:null list)
                    loop
                    ((:lambda (cdr)
                        (:if (core:consp cdr)
                             cdr
                             loop))
                     (mu:cdr loop))))
            list)
           (core:raise list 'common:last "not a list"))))

(mu:intern common "count"
   (:lambda (item list)
     (:if (core:listp list)
          (core:%foldl          
           (:lambda (elt acc)
             (:if (mu:eq elt item)
                  (common:1+ acc)
                  acc))
           0
           list)
          (core:raise list 'common:count "not a list"))))
