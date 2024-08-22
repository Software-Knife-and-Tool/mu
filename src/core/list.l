;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;
(mu:intern core "dotted-pair-p"
  (:lambda (list)
    (:if (core:consp list)
         (core:atom (mu:cdr list))
         ())))

;;;
;;; finds
;;;
(mu:intern core "findl"
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
                (core:raise list 'core:findl "not a list"))
           (core:raise function 'core:findl "not a function"))))

(mu:intern core "findr"
   (:lambda (function list)
     (core:findl function (core:reverse list))))

;;;
;;; positions
;;;
(mu:intern core "positionl"
   (:lambda (function list)
     (:if (core:functionp function)
          (:if (core:listp list)
               (:if list
                    ((:lambda (length)
                        (mu:car (mu:fix
                           (:lambda (loop)
                              (:if (core:listp loop)
                                   loop
                                   (:if (mu:less-than length loop)
                                        ()
                                        (:if (core:%apply function (mu:cons (mu:nth loop list) ()))
                                             (mu:cons loop  ())
                                             (core:1+ loop)))))
                           0)))
                     (mu:length list))
                    ())
               (core:raise list 'core:positionl "not a list"))
          (core:raise function 'core:positionl "not a function"))))

(mu:intern core "positionr"
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
                                         (core:1- loop)))))
                       (mu:difference (mu:length list) 1))))
                (core:raise list 'core:positionr "not a list"))
           (core:raise function 'core:positionr "not a function"))))

;;;
;;; fixed argument list functions
;;;
(mu:intern core "append"
   (:lambda (list)
      (:if (core:listp list)
           (core:foldr
            (:lambda (el acc)
               (:if (core:listp el)
                    (core:foldr mu:cons acc el)
                    el))
            ()
            list)
      (core:raise list 'core:append "not a list"))))

(mu:intern core "reverse"
   (:lambda (list)
      (:if (core:listp list)
           (core:%foldl mu:cons () list)
           (core:raise list 'core:reverse "not a list"))))

(mu:intern core "dropl"
   (:lambda (list n)
      (:if (core:listp list)
         (:if (core:fixnump n)
              (mu:nthcdr n list)
              (core:raise n 'core:dropl "not a fixnum"))
         (core:raise list 'core:dropl "not a list"))))

(mu:intern core "dropr"
   (:lambda (list n)
      (:if (core:listp list)
           (:if (core:fixnump n)
                (core:%foldl
                 mu:cons
                 ()
                 ((:lambda (length)
                     (core:%foldl
                      (:lambda (el last)
                         (:if (mu:less-than (mu:length last) (mu:difference length n))
                              (mu:cons el last)
                              last))
                      ()
                      list))
                  (mu:length list)))
                (core:raise n 'core:dropr "not a fixnum"))
           (core:raise list 'core:dropr "not a list"))))

(mu:intern core "last"
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
           (core:raise list 'core:last "not a list"))))

(mu:intern core "count"
   (:lambda (item list)
     (:if (core:listp list)
          (core:foldl          
           (:lambda (elt acc)
              (:if (mu:eq elt item)
                   (core:1+ acc)
                   acc))
           0
           list)
          (core:raise list 'core:count "not a list"))))
