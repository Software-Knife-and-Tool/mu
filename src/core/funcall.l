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

(mu:intern core "%lambda-arg-list"
   (:lambda (function arg-list)
      (:if (core:%closurep function)
           ((:lambda (rest nreqs)
              (:if (core:%andf rest (mu:eq 0 nreqs))
                   `(mu:cons ,(core:%arg-list arg-list) ())
                   ((:lambda (reqs rest)
                      (:if (core:%andf (core:null rest) (core:null reqs))
                           ()
                           (core:%arg-list `(,@reqs ,(core:%arg-list rest)))))
                    (core:%dropr arg-list (mu:sub (mu:length arg-list) nreqs))
                    (core:%dropl arg-list nreqs))))
            (core:%closure-prop :rest function)
            (core:%closure-prop :arity function))
           (core:%arg-list arg-list))))

(mu:intern core "%quoted-lambda-arg-list"
  (:lambda (fn args)
    (:if (core:%closurep fn)
         (:if (core:%closure-prop :rest fn)
              (:if (core:%andf (core:%closure-prop :rest fn) (mu:eq 0 (core:%closure-prop :arity fn)))
                   (mu:cons args ())
                   ((:lambda (reqs rest)
                      (core:%append
                       reqs
                       (mu:cons rest ())))
                    (core:%dropr args (mu:sub (mu:length args) (core:%closure-prop :arity fn)))
                    (core:%dropl args (core:%closure-prop :arity fn))))
              args)
         args)))

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
     (core:%compile-lambda-arg-list function (core:%mapcar (:lambda (elt) (core:%compile elt env)) arg-list))))

;;;
;;; compile function applications to mu forms
;;;
(mu:intern core "compile-funcall"
   (:lambda (function-form arg-list env)
      (:if (core:%consp function-form)
           ((:lambda (compiled-function)
              (:if (core:functionp compiled-function)
                   (:if (core:%closurep compiled-function)
                        `(,core:%fn-apply-quoted ,compiled-function ,(core:%compile-lambda-arg-list compiled-function arg-list env))
                        `(mu:apply ,compiled-function ,(core:%compile-arg-list arg-list env)))
                   (core:raise compiled-function 'core:compile-funcall "illegal function call")))
            (core:%compile function-form env))
           (:if (mu:boundp function-form)
                ((:lambda (function)
                   (:if (core:functionp function)
                        (:if (core:%closurep function)
                             `(,core:%fn-apply-quoted ,function ,(core:%compile-lambda-arg-list function arg-list env))
                             `(mu:apply ,function ,(core:%compile-arg-list arg-list env)))
                        (core:raise function 'core:compile-funcall "illegal function call")))
                 (mu:symbol-value function-form))
                `(,core:%apply ,function-form ,(core:%compile-arg-list arg-list env))))))

;;;
;;; apply
;;;
(mu:intern core "%fn-apply"
   (:lambda (fn arg-list)
     ((:lambda (env)
        (core:%mapc mu:%frame-push env)
         ((:lambda (value)
            (core:%mapc (:lambda (frame) (mu:%frame-pop (mu:car frame))) env)
            value)
          ((:lambda (lib-fn frame-fn)
             ;;; (core:warn (mu:apply frame-fn `(,lib-fn ,arg-list)) "apply")
             (mu:apply lib-fn (mu:eval (core:%lambda-arg-list fn arg-list))))
           (core:%closure-prop :fn fn)
           (core:%lambda-prop :frame (core:%closure-prop :lambda fn)))))
       (core:%closure-prop :env fn))))

(mu:intern core "%fn-apply-quoted"
   (:lambda (fn arg-list)
      ((:lambda (env)
        (core:%mapc mu:%frame-push env)
        ((:lambda (value)
           (core:%mapc (:lambda (frame) (mu:%frame-pop (mu:car frame))) env)
           value)
         ((:lambda (lib-fn)
             (mu:apply lib-fn arg-list))
          (core:%closure-prop :fn fn))))
        (core:%closure-prop :env fn))))

(mu:intern core "%apply"
   (:lambda (function arg-list)
     (:if (core:%closurep function)
          (core:%fn-apply function arg-list)
          (mu:apply function arg-list))))

(mu:intern core "apply"
   (:lambda (function arg-list)
     (:if (core:functionp function)
          (core:%apply function arg-list)
          (core:raise function 'core:apply "not a function designator"))))
