;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;
(libcore:intern :prelude "consp" (:lambda (t) (libcore:eq :cons (libcore:type-of t))))
(libcore:intern :prelude "null" (:lambda (t) (libcore:eq () t)))
(libcore:intern :prelude "listp" (:lambda (t) (:if (prelude:consp t) :t (prelude:null t))))

(libcore:intern :prelude "dotted-pair-p"
  (:lambda (list)
    (:if (prelude:consp list)
         (prelude:atom (libcore:cdr list))
         ())))

;;;
;;; finds
;;;
(libcore:intern :prelude "findl"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (libcore:cdr
                 (libcore:fix
                  (:lambda (loop)
                     (:if (libcore:eq :t (libcore:car loop))
                          loop
                          (:if (libcore:cdr loop)
                               ((:lambda (el)
                                   (:if (prelude:apply function `(,el))
                                        `(:t ,@el)
                                        `(() ,@(libcore:nthcdr 2 loop))))
                                (libcore:nth 1 loop))
                               loop)))
                  `(() ,@list)))
                (prelude:raise list 'prelude:findl "not a list"))
           (prelude:raise function 'prelude:findl "not a function"))))

(libcore:intern :prelude "findr"
   (:lambda (function list)
     (prelude:findl function (prelude:reverse list))))

;;;
;;; position
;;;
(libcore:intern :prelude "positionl"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (:if list
                    ((:lambda (length)
                        (libcore:car (libcore:fix
                           (:lambda (loop)
                              (:if (prelude:listp loop)
                                   loop
                                   (:if (libcore:fx-lt length loop)
                                        ()
                                        (:if (prelude:apply function `(,(libcore:nth loop list)))
                                             `(,loop)
                                             (prelude:1+ loop)))))
                           0)))
                     (libcore:length list))
                    ())
               (prelude:raise list 'prelude:positionl "not a list"))
          (prelude:raise function 'prelude:positionl "not a function"))))

(libcore:intern :prelude "positionr"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (:if (prelude:null list)
                     ()
                     (libcore:car
                      (libcore:fix
                       (:lambda (loop)
                          (:if (prelude:listp loop)
                               loop
                               (:if (libcore:fx-lt loop 0)
                                    ()
                                    (:if (prelude:apply function `(,(libcore:nth loop list)))
                                         `(,loop)
                                         (prelude:1- loop)))))
                       (libcore:fx-sub (libcore:length list) 1))))
                (prelude:raise list 'prelude:positionr "not a list"))
           (prelude:raise function 'prelude:positionr "not a function"))))

;;;
;;; fixed argument list functions
;;;
(libcore:intern :prelude "append"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:foldr
            (:lambda (el acc)
               (:if (prelude:listp el)
                    (prelude:foldr libcore:cons acc el)
                    el))
            ()
            list)
      (prelude:raise list 'prelude:append "not a list"))))

(libcore:intern :prelude "reverse"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:foldl libcore:cons () list)
           (prelude:raise list 'prelude:reverse "not a list"))))

(libcore:intern :prelude "dropl"
   (:lambda (list n)
      (:if (prelude:listp list)
         (:if (prelude:fixnump n)
              (libcore:nthcdr n list)
              (prelude:raise n 'prelude:dropl "not a fixnum"))
         (prelude:raise list 'prelude:dropl "not a list"))))

(libcore:intern :prelude "dropr"
   (:lambda (list n)
      (:if (prelude:listp list)
           (:if (prelude:fixnump n)
                (prelude:foldl
                 libcore:cons
                 ()
                 ((:lambda (length)
                     (prelude:foldl
                      (:lambda (el last)
                         (:if (libcore:fx-lt (libcore:length last) (libcore:fx-sub length n))
                              `(,el ,@last)
                              last))
                      ()
                      list))
                  (libcore:length list)))
                (prelude:raise n 'prelude:dropr "not a fixnum"))
           (prelude:raise list 'prelude:dropr "not a list"))))

(libcore:intern :prelude "last"
   (:lambda (list)
      (:if (prelude:listp list)
           (libcore:fix
            (:lambda (loop)
               (:if (prelude:null list)
                    loop
                    ((:lambda (cdr)
                        (:if (prelude:consp cdr)
                             cdr
                             loop))
                     (libcore:cdr loop))))
            list)
           (prelude:raise list 'prelude:last "not a list"))))

;;;
;;; count
;;;
(libcore:intern :prelude "count"
   (:lambda (item list)
     (:if (prelude:listp list)
          (prelude:foldl          
           (:lambda (elt acc)
              (:if (libcore:eq elt item)
                   (prelude:1+ acc)
                   acc))
           0
           list)
          (prelude:raise list 'prelude:count "not a list"))))
