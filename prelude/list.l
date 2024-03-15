;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;
(mu:intern :prelude "consp" (:lambda (t) (mu:eq :cons (mu:type-of t))))
(mu:intern :prelude "null" (:lambda (t) (mu:eq () t)))
(mu:intern :prelude "listp" (:lambda (t) (:if (prelude:consp t) :t (prelude:null t))))

(mu:intern :prelude "dotted-pair-p"
  (:lambda (list)
    (:if (prelude:consp list)
         (prelude:atom (mu:cdr list))
         ())))

;;;
;;; finds
;;;
(mu:intern :prelude "findl"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (mu:cdr
                 (mu:fix
                  (:lambda (loop)
                     (:if (mu:eq :t (mu:car loop))
                          loop
                          (:if (mu:cdr loop)
                               ((:lambda (el)
                                   (:if (prelude:apply function `(,el))
                                        `(:t ,@el)
                                        `(() ,@(mu:nthcdr 2 loop))))
                                (mu:nth 1 loop))
                               loop)))
                  `(() ,@list)))
                (prelude:raise list 'prelude:findl "not a list"))
           (prelude:raise function 'prelude:findl "not a function"))))

(mu:intern :prelude "findr"
   (:lambda (function list)
     (prelude:findl function (prelude:reverse list))))

;;;
;;; position
;;;
(mu:intern :prelude "positionl"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (:if list
                    ((:lambda (length)
                        (mu:car (mu:fix
                           (:lambda (loop)
                              (:if (prelude:listp loop)
                                   loop
                                   (:if (mu:fx-lt length loop)
                                        ()
                                        (:if (prelude:apply function `(,(mu:nth loop list)))
                                             `(,loop)
                                             (prelude:1+ loop)))))
                           0)))
                     (mu:length list))
                    ())
               (prelude:raise list 'prelude:positionl "not a list"))
          (prelude:raise function 'prelude:positionl "not a function"))))

(mu:intern :prelude "positionr"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (:if (prelude:null list)
                     ()
                     (mu:car
                      (mu:fix
                       (:lambda (loop)
                          (:if (prelude:listp loop)
                               loop
                               (:if (mu:fx-lt loop 0)
                                    ()
                                    (:if (prelude:apply function `(,(mu:nth loop list)))
                                         `(,loop)
                                         (prelude:1- loop)))))
                       (mu:fx-sub (mu:length list) 1))))
                (prelude:raise list 'prelude:positionr "not a list"))
           (prelude:raise function 'prelude:positionr "not a function"))))

;;;
;;; fixed argument list functions
;;;
(mu:intern :prelude "append"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:foldr
            (:lambda (el acc)
               (:if (prelude:listp el)
                    (prelude:foldr mu:cons acc el)
                    el))
            ()
            list)
      (prelude:raise list 'prelude:append "not a list"))))

(mu:intern :prelude "reverse"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:foldl mu:cons () list)
           (prelude:raise list 'prelude:reverse "not a list"))))

(mu:intern :prelude "dropl"
   (:lambda (list n)
      (:if (prelude:listp list)
         (:if (prelude:fixnump n)
              (mu:nthcdr n list)
              (prelude:raise n 'prelude:dropl "not a fixnum"))
         (prelude:raise list 'prelude:dropl "not a list"))))

(mu:intern :prelude "dropr"
   (:lambda (list n)
      (:if (prelude:listp list)
           (:if (prelude:fixnump n)
                (prelude:foldl
                 mu:cons
                 ()
                 ((:lambda (length)
                     (prelude:foldl
                      (:lambda (el last)
                         (:if (mu:fx-lt (mu:length last) (mu:fx-sub length n))
                              `(,el ,@last)
                              last))
                      ()
                      list))
                  (mu:length list)))
                (prelude:raise n 'prelude:dropr "not a fixnum"))
           (prelude:raise list 'prelude:dropr "not a list"))))

(mu:intern :prelude "last"
   (:lambda (list)
      (:if (prelude:listp list)
           (mu:fix
            (:lambda (loop)
               (:if (prelude:null list)
                    loop
                    ((:lambda (cdr)
                        (:if (prelude:consp cdr)
                             cdr
                             loop))
                     (mu:cdr loop))))
            list)
           (prelude:raise list 'prelude:last "not a list"))))

;;;
;;; count
;;;
(mu:intern :prelude "count"
   (:lambda (item list)
     (:if (prelude:listp list)
          (prelude:foldl          
           (:lambda (elt acc)
              (:if (mu:eq elt item)
                   (prelude:1+ acc)
                   acc))
           0
           list)
          (prelude:raise list 'prelude:count "not a list"))))
