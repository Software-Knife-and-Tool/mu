;;;  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; maps
;;;
(mu:intern common "%nths"
   (:lambda (nth lists)
     (core:%foldl
      (:lambda (el acc)
        (core:%append `(,acc ,(mu:cons (mu:nth nth el) ()))))
      ()
      lists)))

(mu:intern common "%nthcdrs"
   (:lambda (nth lists)
     (core:%foldl
      (:lambda (el acc)
        (core:%append `(,acc ,(mu:cons (mu:nthcdr nth el) ()))))
      ()
      lists)))

(mu:intern common "%mapc"
   (core:compile
    '(%lambda (function &rest lists)
       ((:lambda (min-length)
          (mu:fix
           (:lambda (nth)
             (:if (mu:less-than nth min-length)
                  ((:lambda ()
                   (core:apply function (common:%nths nth lists))
                   (mu:add 1 nth)))
                  nth))
           0)
          (mu:car lists))
        (core:apply common:min (core:%mapcar (:lambda (list) (mu:length list)) lists))))))

(mu:intern common "%mapcar"
   (core:compile
    '(%lambda (function &rest lists)
       ((:lambda (min-length lists function)
          (mu:cdr (mu:cdr
                   (mu:fix
                    (:lambda (loop)
                      ((:lambda (nth args)
                         (:if (mu:less-than nth min-length)
                              (:if (mu:eq :func (mu:type-of function))
                                   (mu:cons (mu:add 1 nth)
                                            (core:%append
                                             `(,args
                                               ,(mu:cons (mu:apply function (common:%nths nth lists))
                                                         ()))))                             
                                   (mu:cons (mu:add 1 nth)
                                            (core:%append
                                             `(,args
                                               ,(mu:cons (core:%apply function (common:%nths nth lists))
                                                         ())))))
                              loop))
                       (mu:car loop)
                       (mu:cdr loop)))
                    '(0 ())))))
        (core:apply common:min (core:%mapcar (:lambda (list) (mu:length list)) lists))
        lists
        function))))

(mu:intern common "%mapl"
   (core:compile
    '(%lambda (function &rest lists)
       ((:lambda (min-length)
          (mu:fix
           (:lambda (nth)
             (:if (mu:less-than nth min-length)
                  ((:lambda ()
                   (core:apply function (common:%nthcdrs nth lists))
                   (mu:add 1 nth)))
                  nth))
           0)
          (mu:car lists))
        (core:apply common:min (core:%mapcar (:lambda (list) (mu:length list)) lists))))))

(mu:intern common "%maplist"
   (core:compile
    '(%lambda (function &rest lists)
       ((:lambda (min-length lists function)
          (mu:cdr (mu:cdr
                   (mu:fix
                    (:lambda (loop)
                      ((:lambda (nth args)
                         (:if (mu:less-than nth min-length)
                              (mu:cons (mu:add 1 nth)
                                       (core:%append `(,args ,(mu:cons (core:apply function (common:%nthcdrs nth lists)) ()))))
                              loop))
                       (mu:car loop)
                       (mu:cdr loop)))
                    '(0 ())))))
        (core:apply common:min (core:%mapcar (:lambda (list) (mu:length list)) lists))
        lists
        function))))
