;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;
(mu:intern common "%last"
   (:lambda (list)
     (mu:fix
      (:lambda (loop)
        (:if (core:null list)
             loop
             ((:lambda (cdr)
                (:if (core:consp cdr)
                     cdr
                     loop))
              (mu:cdr loop))))
      list)))

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
