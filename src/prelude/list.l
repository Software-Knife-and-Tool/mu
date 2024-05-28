;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;
(crux:intern prelude "consp" (:lambda (t) (crux:eq :cons (crux:type-of t))))
(crux:intern prelude "null" (:lambda (t) (crux:eq () t)))
(crux:intern prelude "listp" (:lambda (t) (:if (prelude:consp t) :t (prelude:null t))))

(crux:intern prelude "dotted-pair-p"
  (:lambda (list)
    (:if (prelude:consp list)
         (prelude:atom (crux:cdr list))
         ())))

;;;
;;; finds
;;;
(crux:intern prelude "findl"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (crux:cdr
                 (crux:fix
                  (:lambda (loop)
                     (:if (crux:eq :t (crux:car loop))
                          loop
                          (:if (crux:cdr loop)
                               ((:lambda (el)
                                   (:if (prelude:apply function `(,el))
                                        `(:t ,@el)
                                        `(() ,@(crux:nthcdr 2 loop))))
                                (crux:nth 1 loop))
                               loop)))
                  `(() ,@list)))
                (prelude:raise list 'prelude:findl "not a list"))
           (prelude:raise function 'prelude:findl "not a function"))))

(crux:intern prelude "findr"
   (:lambda (function list)
     (prelude:findl function (prelude:reverse list))))

;;;
;;; position
;;;
(crux:intern prelude "positionl"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (:if list
                    ((:lambda (length)
                        (crux:car (crux:fix
                           (:lambda (loop)
                              (:if (prelude:listp loop)
                                   loop
                                   (:if (crux:less-than length loop)
                                        ()
                                        (:if (prelude:apply function `(,(crux:nth loop list)))
                                             `(,loop)
                                             (prelude:1+ loop)))))
                           0)))
                     (crux:length list))
                    ())
               (prelude:raise list 'prelude:positionl "not a list"))
          (prelude:raise function 'prelude:positionl "not a function"))))

(crux:intern prelude "positionr"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (:if (prelude:null list)
                     ()
                     (crux:car
                      (crux:fix
                       (:lambda (loop)
                          (:if (prelude:listp loop)
                               loop
                               (:if (crux:less-than loop 0)
                                    ()
                                    (:if (prelude:apply function `(,(crux:nth loop list)))
                                         `(,loop)
                                         (prelude:1- loop)))))
                       (crux:difference (crux:length list) 1))))
                (prelude:raise list 'prelude:positionr "not a list"))
           (prelude:raise function 'prelude:positionr "not a function"))))

;;;
;;; fixed argument list functions
;;;
(crux:intern prelude "append"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:foldr
            (:lambda (el acc)
               (:if (prelude:listp el)
                    (prelude:foldr crux:cons acc el)
                    el))
            ()
            list)
      (prelude:raise list 'prelude:append "not a list"))))

(crux:intern prelude "reverse"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:foldl crux:cons () list)
           (prelude:raise list 'prelude:reverse "not a list"))))

(crux:intern prelude "dropl"
   (:lambda (list n)
      (:if (prelude:listp list)
         (:if (prelude:fixnump n)
              (crux:nthcdr n list)
              (prelude:raise n 'prelude:dropl "not a fixnum"))
         (prelude:raise list 'prelude:dropl "not a list"))))

(crux:intern prelude "dropr"
   (:lambda (list n)
      (:if (prelude:listp list)
           (:if (prelude:fixnump n)
                (prelude:foldl
                 crux:cons
                 ()
                 ((:lambda (length)
                     (prelude:foldl
                      (:lambda (el last)
                         (:if (crux:less-than (crux:length last) (crux:difference length n))
                              `(,el ,@last)
                              last))
                      ()
                      list))
                  (crux:length list)))
                (prelude:raise n 'prelude:dropr "not a fixnum"))
           (prelude:raise list 'prelude:dropr "not a list"))))

(crux:intern prelude "last"
   (:lambda (list)
      (:if (prelude:listp list)
           (crux:fix
            (:lambda (loop)
               (:if (prelude:null list)
                    loop
                    ((:lambda (cdr)
                        (:if (prelude:consp cdr)
                             cdr
                             loop))
                     (crux:cdr loop))))
            list)
           (prelude:raise list 'prelude:last "not a list"))))

;;;
;;; count
;;;
(crux:intern prelude "count"
   (:lambda (item list)
     (:if (prelude:listp list)
          (prelude:foldl          
           (:lambda (elt acc)
              (:if (crux:eq elt item)
                   (prelude:1+ acc)
                   acc))
           0
           list)
          (prelude:raise list 'prelude:count "not a list"))))
