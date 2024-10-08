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
          (core:%mapcar common:identity list)             
          (core:raise list 'common:copy-list "not a list"))))

;;;
;;; cxxxxr
;;;
(mu:intern common "caar" (:lambda (x) (mu:car (mu:car x))))
(mu:intern common "cadr" (:lambda (x) (mu:car (mu:cdr x))))  
(mu:intern common "cdar" (:lambda (x) (mu:cdr (mu:car x))))  
(mu:intern common "cddr" (:lambda (x) (mu:cdr (mu:cdr x))))  
(mu:intern common "caaar" (:lambda (x) (mu:car (mu:car (mu:car x)))))  
(mu:intern common "caadr" (:lambda (x) (mu:car (mu:car (mu:cdr x)))))  
(mu:intern common "cadar" (:lambda (x) (mu:car (mu:cdr (mu:car x))))) 
(mu:intern common "caddr" (:lambda (x) (mu:car (mu:cdr (mu:cdr x)))))  
(mu:intern common "cdaar" (:lambda (x) (mu:cdr (mu:car (mu:car x)))))  
(mu:intern common "cdadr" (:lambda (x) (mu:cdr (mu:car (mu:cdr x)))))  
(mu:intern common "cddar" (:lambda (x) (mu:cdr (mu:cdr (mu:car x))))) 
(mu:intern common "cdddr" (:lambda (x) (mu:cdr (mu:cdr (mu:cdr x))))) 
(mu:intern common "caaaar" (:lambda (x) (mu:car (mu:car (mu:car (mu:car x))))))
(mu:intern common "caaadr" (:lambda (x) (mu:car (mu:car (mu:car (mu:cdr x)))))) 
(mu:intern common "caadar" (:lambda (x) (mu:car (mu:car (mu:cdr (mu:car x)))))) 
(mu:intern common "caaddr" (:lambda (x) (mu:car (mu:car (mu:cdr (mu:cdr x)))))) 
(mu:intern common "cadaar" (:lambda (x) (mu:car (mu:cdr (mu:car (mu:car x)))))) 
(mu:intern common "cadadr" (:lambda (x) (mu:car (mu:cdr (mu:car (mu:cdr x)))))) 
(mu:intern common "caddar" (:lambda (x) (mu:car (mu:cdr (mu:cdr (mu:car x)))))) 
(mu:intern common "cadddr" (:lambda (x) (mu:car (mu:cdr (mu:cdr (mu:cdr x)))))) 
(mu:intern common "cdaaar" (:lambda (x) (mu:cdr (mu:car (mu:car (mu:car x)))))) 
(mu:intern common "cdaadr" (:lambda (x) (mu:cdr (mu:car (mu:car (mu:cdr x)))))) 
(mu:intern common "cdadar" (:lambda (x) (mu:cdr (mu:car (mu:cdr (mu:car x)))))) 
(mu:intern common "cdaddr" (:lambda (x) (mu:cdr (mu:car (mu:cdr (mu:cdr x)))))) 
(mu:intern common "cddaar" (:lambda (x) (mu:cdr (mu:cdr (mu:car (mu:car x)))))) 
(mu:intern common "cddadr" (:lambda (x) (mu:cdr (mu:cdr (mu:car (mu:cdr x)))))) 
(mu:intern common "cdddar" (:lambda (x) (mu:cdr (mu:cdr (mu:cdr (mu:car x))))))
(mu:intern common "cddddr" (:lambda (x) (mu:cdr (mu:cdr (mu:cdr (mu:cdr x))))) )

(mu:intern common "first" (:lambda (x) (mu:car x)))
(mu:intern common "second" (:lambda (x) (mu:car (mu:cdr x))))
(mu:intern common "third" (:lambda (x) (mu:car (common:cddr x))))
(mu:intern common "fourth" (:lambda (x) (mu:car (common:cdddr x))))
(mu:intern common "fifth" (:lambda (x) (mu:car (common:cddddr x))))
(mu:intern common "sixth" (:lambda (x) (mu:car (mu:cdr (common:cddddr x)))))
(mu:intern common "seventh" (:lambda (x) (mu:car (common:cddr (common:cddddr x)))))
(mu:intern common "eighth" (:lambda (x) (mu:car (common:cdddr (common:cddddr x)))))
(mu:intern common "ninth" (:lambda (x) (mu:car (common:cddddr (common:cddddr x)))))
(mu:intern common "tenth" (:lambda (x) (mu:car (mu:cdr (common:cddddr (common:cddddr x))))))
