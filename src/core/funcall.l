;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; compile function applications
;;;

;;;
;;; argument lists
;;;
(mu:intern core "%arg-list"
   (:lambda (arg-list)
      (core:%foldr
       (:lambda (elt acc)
         `(mu:cons ,elt ,acc))
       ()
       arg-list)))

(mu:intern core "%quoted-arg-list"
   (:lambda (arg-list)
      (core:%foldr
       (:lambda (elt acc)
         `(mu:cons ,(mu:cons :quote (mu:cons elt ())) ,acc))
       ()
       arg-list)))

(mu:intern core "%lambda-arg-list"
   (:lambda (function arg-list)
     (:if (core:%core-function-p function)
           ((:lambda (arity rest nreqs)
              (:if rest
                   (:if (mu:eq 0 nreqs)
                        `(mu:cons ,(core:%arg-list arg-list) ())
                        ((:lambda (req-list rest-list)
                           (:if (core:logand (core:null rest-list) (core:null req-list))
                                ()
                                (core:%arg-list `(,@req-list ,(core:%arg-list rest-list)))))
                         (core:%dropr arg-list (mu:sub (mu:length arg-list) arity))
                         (core:%dropl arg-list arity)))
                   (core:%arg-list arg-list)))
            (core:%core-function-prop :arity function)
            (core:%core-function-prop :rest function)
            (mu:length (core:%core-function-prop :require function)))
           (core:%arg-list arg-list))))

(mu:intern core "%quoted-lambda-arg-list"
   (:lambda (function arg-list)
     (:if (core:%core-function-p function)
           ((:lambda (arity rest nreqs)
              (:if rest
                   (:if (mu:eq 0 nreqs)
                        `(mu:cons ,arg-list ())
                        ((:lambda (req-list rest-list)
                           (:if (core:logand (core:null rest-list) (core:null req-list))
                                ()
                                (core:%quoted-arg-list `(,@req-list ,rest-list))))
                         (core:%dropr arg-list (mu:sub (mu:length arg-list) arity))
                         (core:%dropl arg-list arity)))
                   (core:%quoted-arg-list arg-list)))
            (core:%core-function-prop :arity function)
            (core:%core-function-prop :rest function)
            (mu:length (core:%core-function-prop :require function)))
           (core:%quoted-arg-list arg-list))))

#|
(mu:intern core "%quoted-lambda-arg-list"
   (:lambda (fn args)
     (:if (core:%core-function-p fn)
          (:if (core:%core-function-prop :rest fn)
               (:if (core:logand (core:%core-function-prop :rest fn) (mu:eq 0 (core:%core-function-prop :arity fn)))
                    (mu:cons args ())
                    ((:lambda (reqs rest)
                       (:if (core:logand (core:null rest) (core:null reqs))
                            ()
                            (core:%arg-list `(,@reqs ,(core:%arg-list rest)))))
                     (core:%dropr args (mu:sub (mu:length args) (core:%core-function-prop :arity fn)))
                     (core:%dropl args (mu:length (core:%core-function-prop :require function)))))
               args)
          args)))
|#

;;;
;;; compile argument lists
;;;
(mu:intern core "%compile-arg-list"
   (:lambda (arg-list env)
      (core:%foldr
       (:lambda (elt acc)
          `(mu:cons ,@(mu:cons elt `(,acc))))
       ()
       (core:%mapcar
        (:lambda (form)
           (core:%compile form env))
        arg-list))))

(mu:intern core "%compile-lambda-arg-list"
   (:lambda (function arg-list env)
     (core:%lambda-arg-list function (core:%mapcar (:lambda (elt) (core:%compile elt env)) arg-list))))

(mu:intern core "%compile-quoted-lambda-arg-list"
   (:lambda (function arg-list env)
     (core:%quoted-lambda-arg-list function arg-list)))
     
;;;
;;; compile function application to mu form
;;;
;;; ((lambda lambda-list . body) . arg-list) env
;;; core-function env
;;; symbol env
;;;
(mu:intern core "compile-funcall"
   (:lambda (function-form arg-list env)
     (:if (core:%consp function-form)
          ((:lambda (function)
             (:if (core:logor (core:functionp function) (core:%consp function))
                  (:if (core:%core-function-p function)
                       ((:lambda (mu-form)
                          (:if (core:%core-function-p function)
                               `(,core:%fapply
                                 ,function
                                 ,mu-form
                                 ,(mu:cons (core:%compile-lambda-arg-list function arg-list env) ()))
                               `(,core:%fapply
                                 ,function
                                 ,mu-form
                                 ,(core:%compile-arg-list arg-list env))))
                        (core:%core-function-prop :mu function))
                       `(mu:apply ,function ,(core:%compile-arg-list arg-list env)))
                  (core:raise function 'core:compile-funcall "not a function designator")))
           (core:%compile function-form env))
          (:if (mu:boundp function-form)
               ((:lambda (function)
                  (:if (core:functionp function)
                       (:if (core:%core-function-p function)
                            `(,core:%fapply
                              ,function
                              ,(core:%core-function-prop :mu function)
                              ,(core:%compile-lambda-arg-list function arg-list env))
                            `(mu:apply ,function ,(core:%compile-arg-list arg-list env)))
                       (core:raise function 'core:compile-funcall "not a function designator")))
                (mu:symbol-value function-form))
               `(,core:%apply ,function-form ,(core:%compile-arg-list arg-list env))))))
