;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; let compiler
;;;
;;;
;;; compile closure
;;;
(mu:intern core "%compile-let"
   (:lambda (lambda body env)
     (:if (core:find-if (:lambda (el) (core:null (mu:eq :symbol (mu:type-of el)))) lambda)
          (mu:raise-from lambda 'core:%compile-closure :syntax)
          ((:lambda (desc)
             ((:lambda (env)
                ((:lambda (body)
                   ((:lambda (closure)
                      (core:%add-frame (core:%env-prop :name (mu:car env)) (core:%closure-prop :mu closure))
                      closure)
                    (core:%make-closure (mu:car desc) (mu:cdr desc) `(:lambda ,(mu:car desc) ,@body) () ())))
                 (core:mapcar
                  (:lambda (form)
                    (core:%compile form env))
                  body)))
              (mu:cons (core:%make-env (mu:length env) (mu:car desc) (core:gensym)) env)))
          ((:lambda (desc)
             (:if (core:%or (core:fixnump desc) (core:null body))
                  (core:%list lambda)
                  desc))
           (core:foldl
            (:lambda (el acc)
              (:if (core:numberp acc)
                   (:if (mu:eq '&rest el)
                        (:if (mu:eq (mu:length lambda) (mu:add 2 acc))
                             (mu:cons
                              `(,@(core:%dropr lambda 2) ,@(core:dropl lambda (mu:sub (mu:length lambda) 1)))
                              (mu:nth (mu:sub (mu:length lambda) 1) lambda))
                             (mu:raise-from lambda 'core:%compile-closure :syntax))
                        (mu:add 1 acc))
                   acc))
            0
            lambda))))))

#|
(core:compile
 '(%defmacro let (binds &rest body)
   `((%lambda ,(core:mapcar mu:car binds) ,@body)
     ,@(core:mapcar (:lambda (bind) (mu:nth 1 bind)) binds))))

(core:compile
 '(%defmacro let* (binds &rest body)
   (%if binds
       `(let (,(mu:car binds)) (let* ,(mu:cdr binds) ,@body))
`(let () ,@body))))
|#
