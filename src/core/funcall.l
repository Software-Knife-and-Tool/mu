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
         `(mu:cons ,(core:quote elt) ,acc))
       ()
       arg-list)))

(mu:intern core "%lambda-arg-list"
   (:lambda (function arg-list)
     (:if (core:%closurep function)
          ((:lambda (arity rest nreqs)
             (:if rest
                  (:if (mu:eq 0 arity)
                       `(mu:cons ,(core:%arg-list arg-list) ())
                       ((:lambda (req-list rest-list)
                          (:if (core:logand (core:null rest-list) (core:null req-list))
                               ()
                               (core:%arg-list `(,@req-list ,(core:%arg-list rest-list)))))
                        (core:%dropr arg-list (mu:sub (mu:length arg-list) arity))
                        (core:%dropl arg-list arity)))
                  (core:%arg-list arg-list)))
           (core:%closure-prop :arity function)
           (core:%closure-prop :rest function)
           (mu:length (core:%closure-prop :require function)))
          (core:%arg-list arg-list))))

(mu:intern core "%quoted-lambda-arg-list"
   (:lambda (function arg-list)
     (:if (core:%closurep function)
          ((:lambda (arity rest nreqs)
              (:if rest
                   (:if (mu:eq 0 arity)
                        `(mu:cons ,(core:%quoted-arg-list arg-list) ())
                        ((:lambda (req-list rest-list)
                           (:if (core:logand (core:null rest-list) (core:null req-list))
                                ()
                                (core:%quoted-arg-list `(,@req-list ,rest-list))))
                         (core:%dropr arg-list (mu:sub (mu:length arg-list) arity))
                         (core:%dropl arg-list arity)))
                   (core:%quoted-arg-list arg-list)))
            (core:%closure-prop :arity function)
            (core:%closure-prop :rest function)
            (mu:length (core:%closure-prop :require function)))
           (core:%quoted-arg-list arg-list))))

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
(mu:intern core "compile-funcall"
   (:lambda (function-form arg-list env)
     (:if (core:%consp function-form)
          ((:lambda (function)
             (:if (core:logor (core:functionp function) (core:%consp function))
                  (:if (core:functionp function)
                       (:if (core:%closurep function)
                            `(,core:%apply
                              ,function
                              ,(core:%closure-prop :form function)
                              ,(core:%compile-lambda-arg-list function arg-list env))
                            `(,mu:apply ,function ,(core:%compile-arg-list arg-list env)))
                  `(mu:apply ,function ,(core:%compile-arg-list arg-list env)))
             (core:raise function 'core:compile-funcall "not a function designator")))
           (core:%compile function-form env))
          (:if (mu:boundp function-form)
               ((:lambda (function)
                  (:if (core:functionp function)
                       (:if (core:%closurep function)
                            `(,core:%apply
                              ,function
                              ,(core:%closure-prop :form function)
                              ,(core:%compile-lambda-arg-list function arg-list env))
                            `(mu:apply ,function ,(core:%compile-arg-list arg-list env)))
                       (core:raise function 'core:compile-funcall "not a function designator")))
                (mu:symbol-value function-form))
               `(,core:apply ,function-form ,(core:%compile-arg-list arg-list env))))))
