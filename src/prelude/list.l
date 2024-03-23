;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;
(core:intern :prelude "consp" (:lambda (t) (core:eq :cons (core:type-of t))))
(core:intern :prelude "null" (:lambda (t) (core:eq () t)))
(core:intern :prelude "listp" (:lambda (t) (:if (prelude:consp t) :t (prelude:null t))))

(core:intern :prelude "dotted-pair-p"
  (:lambda (list)
    (:if (prelude:consp list)
         (prelude:atom (core:cdr list))
         ())))

;;;
;;; finds
;;;
(core:intern :prelude "findl"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (core:cdr
                 (core:fix
                  (:lambda (loop)
                     (:if (core:eq :t (core:car loop))
                          loop
                          (:if (core:cdr loop)
                               ((:lambda (el)
                                   (:if (prelude:apply function `(,el))
                                        `(:t ,@el)
                                        `(() ,@(core:nthcdr 2 loop))))
                                (core:nth 1 loop))
                               loop)))
                  `(() ,@list)))
                (prelude:raise list 'prelude:findl "not a list"))
           (prelude:raise function 'prelude:findl "not a function"))))

(core:intern :prelude "findr"
   (:lambda (function list)
     (prelude:findl function (prelude:reverse list))))

;;;
;;; position
;;;
(core:intern :prelude "positionl"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (:if list
                    ((:lambda (length)
                        (core:car (core:fix
                           (:lambda (loop)
                              (:if (prelude:listp loop)
                                   loop
                                   (:if (core:fx-lt length loop)
                                        ()
                                        (:if (prelude:apply function `(,(core:nth loop list)))
                                             `(,loop)
                                             (prelude:1+ loop)))))
                           0)))
                     (core:length list))
                    ())
               (prelude:raise list 'prelude:positionl "not a list"))
          (prelude:raise function 'prelude:positionl "not a function"))))

(core:intern :prelude "positionr"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (:if (prelude:null list)
                     ()
                     (core:car
                      (core:fix
                       (:lambda (loop)
                          (:if (prelude:listp loop)
                               loop
                               (:if (core:fx-lt loop 0)
                                    ()
                                    (:if (prelude:apply function `(,(core:nth loop list)))
                                         `(,loop)
                                         (prelude:1- loop)))))
                       (core:fx-sub (core:length list) 1))))
                (prelude:raise list 'prelude:positionr "not a list"))
           (prelude:raise function 'prelude:positionr "not a function"))))

;;;
;;; fixed argument list functions
;;;
(core:intern :prelude "append"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:foldr
            (:lambda (el acc)
               (:if (prelude:listp el)
                    (prelude:foldr core:cons acc el)
                    el))
            ()
            list)
      (prelude:raise list 'prelude:append "not a list"))))

(core:intern :prelude "reverse"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:foldl core:cons () list)
           (prelude:raise list 'prelude:reverse "not a list"))))

(core:intern :prelude "dropl"
   (:lambda (list n)
      (:if (prelude:listp list)
         (:if (prelude:fixnump n)
              (core:nthcdr n list)
              (prelude:raise n 'prelude:dropl "not a fixnum"))
         (prelude:raise list 'prelude:dropl "not a list"))))

(core:intern :prelude "dropr"
   (:lambda (list n)
      (:if (prelude:listp list)
           (:if (prelude:fixnump n)
                (prelude:foldl
                 core:cons
                 ()
                 ((:lambda (length)
                     (prelude:foldl
                      (:lambda (el last)
                         (:if (core:fx-lt (core:length last) (core:fx-sub length n))
                              `(,el ,@last)
                              last))
                      ()
                      list))
                  (core:length list)))
                (prelude:raise n 'prelude:dropr "not a fixnum"))
           (prelude:raise list 'prelude:dropr "not a list"))))

(core:intern :prelude "last"
   (:lambda (list)
      (:if (prelude:listp list)
           (core:fix
            (:lambda (loop)
               (:if (prelude:null list)
                    loop
                    ((:lambda (cdr)
                        (:if (prelude:consp cdr)
                             cdr
                             loop))
                     (core:cdr loop))))
            list)
           (prelude:raise list 'prelude:last "not a list"))))

;;;
;;; count
;;;
(core:intern :prelude "count"
   (:lambda (item list)
     (:if (prelude:listp list)
          (prelude:foldl          
           (:lambda (elt acc)
              (:if (core:eq elt item)
                   (prelude:1+ acc)
                   acc))
           0
           list)
          (prelude:raise list 'prelude:count "not a list"))))
