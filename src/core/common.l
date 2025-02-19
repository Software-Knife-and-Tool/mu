;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; forms common across lisp families
;;;
(mu:intern core "identity"
   (:lambda (obj)
     obj))

;;;
;;; rest functions
;;;
(mu:intern core "append"
   (core:compile '(%lambda (&rest list) (mu:append list))))

(mu:intern core "list"
   (core:compile '(%lambda (&rest list) list)))         

(mu:intern core "list*"
   (core:compile
    '(%lambda (&rest args)
      (mu:car
       (mu:fix
        (:lambda (loop)
          ((:lambda (list args)
             (:if args
                  (:if (mu:cdr args)
                       (mu:cons `(,@list ,@(mu:cons (mu:car args) ())) (mu:cdr args))
                       (mu:cons `(,@list ,@(mu:car args)) ()))
                  loop))
           (mu:car loop)
           (mu:cdr loop)))
        (mu:cons () args))))))

(mu:intern core "vector"
   (core:compile
    `(%lambda (&rest list) (mu:make-vector :t list))))

(mu:intern core "format"
   (core:compile
    `(%lambda (stream format &rest args)
       (core:%format stream format args))))

(mu:intern core "funcall"
   (core:compile
    `(%lambda (fn &rest args)
       (core:apply fn args))))

;;;
;;; macros
;;;
(core:compile
 '(%defmacro progn (&rest body)
     (:if (mu:less-than (mu:length body) 2)
       (mu:car body)
       `((%lambda () ,@body)))))

(core:compile
 '(%defmacro when (test &rest body)
   `(%if ,test (progn ,@body))))

(core:compile
 '(%defmacro unless (test &rest body)
   `(%if (core:null ,test) (progn ,@body))))

(core:compile
 '(%defmacro let (binds &rest body)
   `((%lambda ,(core:%mapcar mu:car binds) ,@body)
     ,@(core:%mapcar (:lambda (bind) (mu:nth 1 bind)) binds))))

(core:compile
 '(%defmacro let* (binds &rest body)
   (:if binds
       `(let (,(mu:car binds)) (let* ,(mu:cdr binds) ,@body))
       `(let () ,@body))))

(core:compile
 '(%defmacro cond (&rest clauses)
   (core:%foldr
    (:lambda (clause cond-form)
      ((:lambda (test body)
         `(%if ,test (progn ,@body) ,cond-form))
       (mu:car clause)
       (mu:cdr clause)))
      ()
      clauses)))

(core:compile
 '(%defmacro and (&rest terms)
   (:if terms
    (:if (mu:eq 1 (mu:length terms))
     (mu:car terms)
     (core:%foldl
      (:lambda (term acc)
        `(%if ,acc ,term))
      `(%if ,(mu:nth 0 terms) ,(mu:nth 1 terms))
      (mu:nthcdr 2 terms)))
    :t)))

(core:compile
 '(%defmacro or (&rest terms)
   (:if terms
    (:if (mu:eq 1 (mu:length terms))
     (mu:car terms)
     (core:%foldl
      (:lambda (term acc)
        ((:lambda (g)
           `(,acc (let ((,g ,term)) ,g)))
         (core:gensym)))
      ((:lambda (g1 g2)
         `(let ((,g1 ,(mu:nth 0 terms)))
            ,g1
            (let ((,g2 ,(mu:nth 1 terms)))
              ,g2)))
       (core:gensym)
       (core:gensym))
      (mu:nthcdr 2 terms)))
    ())))
