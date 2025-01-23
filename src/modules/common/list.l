;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;

;;;
;;; maps
;;;
(mu:intern common "mapc"
   (:lambda (function list)
     (:if (core:functionp function)
          (:if (core:listp list)
               (core:%mapc function list)
               (core:raise list 'common:mapc "not a list"))
          (core:raise function 'common:mapc "not a function"))
     list))

(mu:intern common "mapcar"
   (:lambda (function list)
     (:if (core:functionp function)
          (:if (core:listp list)
               (core:%mapcar function list)
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
