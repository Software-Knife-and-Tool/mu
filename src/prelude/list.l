;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;
(lib:intern :prelude "consp" (:lambda (t) (lib:eq :cons (lib:type-of t))))
(lib:intern :prelude "null" (:lambda (t) (lib:eq () t)))
(lib:intern :prelude "listp" (:lambda (t) (:if (prelude:consp t) :t (prelude:null t))))

(lib:intern :prelude "dotted-pair-p"
  (:lambda (list)
    (:if (prelude:consp list)
         (prelude:atom (lib:cdr list))
         ())))

;;;
;;; finds
;;;
(lib:intern :prelude "findl"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (lib:cdr
                 (lib:fix
                  (:lambda (loop)
                     (:if (lib:eq :t (lib:car loop))
                          loop
                          (:if (lib:cdr loop)
                               ((:lambda (el)
                                   (:if (prelude:apply function `(,el))
                                        `(:t ,@el)
                                        `(() ,@(lib:nthcdr 2 loop))))
                                (lib:nth 1 loop))
                               loop)))
                  `(() ,@list)))
                (prelude:raise list 'prelude:findl "not a list"))
           (prelude:raise function 'prelude:findl "not a function"))))

(lib:intern :prelude "findr"
   (:lambda (function list)
     (prelude:findl function (prelude:reverse list))))

;;;
;;; position
;;;
(lib:intern :prelude "positionl"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (:if list
                    ((:lambda (length)
                        (lib:car (lib:fix
                           (:lambda (loop)
                              (:if (prelude:listp loop)
                                   loop
                                   (:if (lib:fx-lt length loop)
                                        ()
                                        (:if (prelude:apply function `(,(lib:nth loop list)))
                                             `(,loop)
                                             (prelude:1+ loop)))))
                           0)))
                     (lib:length list))
                    ())
               (prelude:raise list 'prelude:positionl "not a list"))
          (prelude:raise function 'prelude:positionl "not a function"))))

(lib:intern :prelude "positionr"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (:if (prelude:null list)
                     ()
                     (lib:car
                      (lib:fix
                       (:lambda (loop)
                          (:if (prelude:listp loop)
                               loop
                               (:if (lib:fx-lt loop 0)
                                    ()
                                    (:if (prelude:apply function `(,(lib:nth loop list)))
                                         `(,loop)
                                         (prelude:1- loop)))))
                       (lib:fx-sub (lib:length list) 1))))
                (prelude:raise list 'prelude:positionr "not a list"))
           (prelude:raise function 'prelude:positionr "not a function"))))

;;;
;;; fixed argument list functions
;;;
(lib:intern :prelude "append"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:foldr
            (:lambda (el acc)
               (:if (prelude:listp el)
                    (prelude:foldr lib:cons acc el)
                    el))
            ()
            list)
      (prelude:raise list 'prelude:append "not a list"))))

(lib:intern :prelude "reverse"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:foldl lib:cons () list)
           (prelude:raise list 'prelude:reverse "not a list"))))

(lib:intern :prelude "dropl"
   (:lambda (list n)
      (:if (prelude:listp list)
         (:if (prelude:fixnump n)
              (lib:nthcdr n list)
              (prelude:raise n 'prelude:dropl "not a fixnum"))
         (prelude:raise list 'prelude:dropl "not a list"))))

(lib:intern :prelude "dropr"
   (:lambda (list n)
      (:if (prelude:listp list)
           (:if (prelude:fixnump n)
                (prelude:foldl
                 lib:cons
                 ()
                 ((:lambda (length)
                     (prelude:foldl
                      (:lambda (el last)
                         (:if (lib:fx-lt (lib:length last) (lib:fx-sub length n))
                              `(,el ,@last)
                              last))
                      ()
                      list))
                  (lib:length list)))
                (prelude:raise n 'prelude:dropr "not a fixnum"))
           (prelude:raise list 'prelude:dropr "not a list"))))

(lib:intern :prelude "last"
   (:lambda (list)
      (:if (prelude:listp list)
           (lib:fix
            (:lambda (loop)
               (:if (prelude:null list)
                    loop
                    ((:lambda (cdr)
                        (:if (prelude:consp cdr)
                             cdr
                             loop))
                     (lib:cdr loop))))
            list)
           (prelude:raise list 'prelude:last "not a list"))))

;;;
;;; count
;;;
(lib:intern :prelude "count"
   (:lambda (item list)
     (:if (prelude:listp list)
          (prelude:foldl          
           (:lambda (elt acc)
              (:if (lib:eq elt item)
                   (prelude:1+ acc)
                   acc))
           0
           list)
          (prelude:raise list 'prelude:count "not a list"))))
