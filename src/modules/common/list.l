;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;

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

(mu:intern common "mapc"
   (core:compile
    '(%lambda (function &rest lists)
      (:if lists
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
        (core:apply common:min (core:%mapcar (:lambda (list) (mu:length list)) lists)))
       (core:raise () 'common:mapc "invalid argument")))))

(mu:intern common "mapcar"
   (core:compile
    '(%lambda (function &rest lists)
      (:if lists
       ((:lambda (min-length lists function)
          (mu:cdr (mu:nthcdr 2
                   (mu:fix
                    (:lambda (loop)
                      ((:lambda (nth args)
                         (:if (mu:less-than nth min-length)
                              (mu:cons (mu:add 1 nth)
                                       (core:%append `(,args ,(mu:cons (core:apply function (common:%nths nth lists)) ()))))
                              loop))
                       (mu:car loop)
                       (mu:cdr loop)))
                    `(0 .())))))
        (core:apply common:min (core:%mapcar (:lambda (list) (mu:length list)) lists))
        lists
        function)
       (core:raise () 'common:mapcar "invalid argument")))))

(mu:intern common "mapl"
   (core:compile
    '(%lambda (function &rest lists)
      (:if lists
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
        (core:apply common:min (core:%mapcar (:lambda (list) (mu:length list)) lists)))
       (core:raise () 'common:mapc "invalid argument")))))

(mu:intern common "maplist"
   (core:compile
    '(%lambda (function &rest lists)
      (:if lists
       ((:lambda (min-length lists function)
          (mu:cdr (mu:nthcdr 2
                   (mu:fix
                    (:lambda (loop)
                      ((:lambda (nth args)
                         (:if (mu:less-than nth min-length)
                              (mu:cons (mu:add 1 nth)
                                       (core:%append `(,args ,(mu:cons (core:apply function (common:%nthcdrs nth lists)) ()))))
                              loop))
                       (mu:car loop)
                       (mu:cdr loop)))
                    `(0 .())))))
        (core:apply common:min (core:%mapcar (:lambda (list) (mu:length list)) lists))
        lists
        function)
       (core:raise () 'common:mapcar "invalid argument")))))

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

(mu:intern common "copy-list"
   (:lambda (list)
     (:if (core:listp list)
          (core:%mapcar core:identity list)             
          (core:raise list 'common:copy-list "not a list"))))

;;;
;;; cxxxxr
;;;
(defun cons (x y) (mu:cons x y))
(defun car (x) (mu:car x))
(defun cdr (x) (mu:cdr x))
(defun first (x) (mu:car x))
(defun rest (x) (mu:cdr x))
(defun caar (x) (mu:car (mu:car x)))
(defun cadr (x) (mu:car (mu:cdr x)))
(defun cdar (x) (mu:cdr (mu:car x)))  
(defun cddr (x) (mu:cdr (mu:cdr x)))  
(defun caaar (x) (mu:car (mu:car (mu:car x))))  
(defun caadr (x) (mu:car (mu:car (mu:cdr x))))  
(defun cadar (x) (mu:car (mu:cdr (mu:car x)))) 
(defun caddr (x) (mu:car (mu:cdr (mu:cdr x))))  
(defun cdaar (x) (mu:cdr (mu:car (mu:car x))))  
(defun cdadr (x) (mu:cdr (mu:car (mu:cdr x))))  
(defun cddar (x) (mu:cdr (mu:cdr (mu:car x)))) 
(defun cdddr (x) (mu:cdr (mu:cdr (mu:cdr x)))) 
(defun caaaar (x) (mu:car (mu:car (mu:car (mu:car x)))))
(defun caaadr (x) (mu:car (mu:car (mu:car (mu:cdr x))))) 
(defun caadar (x) (mu:car (mu:car (mu:cdr (mu:car x))))) 
(defun caaddr (x) (mu:car (mu:car (mu:cdr (mu:cdr x))))) 
(defun cadaar (x) (mu:car (mu:cdr (mu:car (mu:car x))))) 
(defun cadadr (x) (mu:car (mu:cdr (mu:car (mu:cdr x))))) 
(defun caddar (x) (mu:car (mu:cdr (mu:cdr (mu:car x))))) 
(defun cadddr (x) (mu:car (mu:cdr (mu:cdr (mu:cdr x))))) 
(defun cdaaar (x) (mu:cdr (mu:car (mu:car (mu:car x))))) 
(defun cdaadr (x) (mu:cdr (mu:car (mu:car (mu:cdr x))))) 
(defun cdadar (x) (mu:cdr (mu:car (mu:cdr (mu:car x))))) 
(defun cdaddr (x) (mu:cdr (mu:car (mu:cdr (mu:cdr x))))) 
(defun cddaar (x) (mu:cdr (mu:cdr (mu:car (mu:car x))))) 
(defun cddadr (x) (mu:cdr (mu:cdr (mu:car (mu:cdr x))))) 
(defun cdddar (x) (mu:cdr (mu:cdr (mu:cdr (mu:car x)))))
(defun cddddr (x) (mu:cdr (mu:cdr (mu:cdr (mu:cdr x)))))
(defun first (x) (mu:car x))
(defun second (x) (mu:car (mu:cdr x)))
(defun third (x) (mu:car (cddr x)))
(defun fourth (x) (mu:car (cdddr x)))
(defun fifth (x) (mu:car (cddddr x)))
(defun sixth (x) (mu:car (mu:cdr (cddddr x))))
(defun seventh (x) (mu:car (cddr (cddddr x))))
(defun eighth (x) (mu:car (cdddr (cddddr x))))
(defun ninth (x) (mu:car (cddddr (cddddr x))))
(defun tenth (x) (mu:car (mu:cdr (cddddr (cddddr x)))))
