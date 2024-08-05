;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; compile function applications
;;;

;;;
;;; argument lists
;;;
(mu:intern prelude "%arg-list"
   (:lambda (arg-list)
      (prelude:foldr
       (:lambda (elt acc)
         `(mu:cons ,elt ,acc))
       ()
       arg-list)))

(mu:intern prelude "%lambda-arg-list"
   (:lambda (function arg-list)
      (:if (prelude:%prelude-function-p function)
           ((:lambda (rest nreqs)
              (:if (prelude:%andf rest (prelude:zerop nreqs))
                   `(mu:cons ,(prelude:%arg-list arg-list) ())
                   ((:lambda (reqs rest)
                      (:if (prelude:%andf (prelude:null rest) (prelude:null reqs))
                           ()
                           (prelude:%arg-list `(,@reqs ,(prelude:%arg-list rest)))))
                    (prelude:dropr arg-list (mu:difference (mu:length arg-list) nreqs))
                    (prelude:dropl arg-list nreqs))))
            (prelude:%fn-prop :rest function)
            (prelude:%fn-prop :arity function))
           (prelude:%arg-list arg-list))))

(mu:intern prelude "%quoted-lambda-arg-list"
  (:lambda (fn args)
    (:if (prelude:%prelude-function-p fn)
         (:if (prelude:%fn-prop :rest fn)
              (:if (prelude:%andf (prelude:%fn-prop :rest fn) (prelude:zerop (prelude:%fn-prop :arity fn)))
                   `(,args)
                   ((:lambda (reqs rest)
                      (prelude:append
                       reqs
                       `(,rest)))
                    (prelude:dropr args (mu:difference (mu:length args) (prelude:%fn-prop :arity fn)))
                    (prelude:dropl args (prelude:%fn-prop :arity fn))))
              args)
         args)))

;;;
;;; compile argument lists
;;;
(mu:intern prelude "%compile-arg-list"
   (:lambda (arg-list env)
      (prelude:foldr
       (:lambda (elt acc)
          `(mu:cons ,@(mu:cons elt `(,acc))))
       ()
       (prelude:mapcar
        (:lambda (form)
           (prelude:%compile form env))
        arg-list))))

(mu:intern prelude "%compile-lambda-arg-list"
   (:lambda (function arg-list env)
     (prelude:%lambda-arg-list function (prelude:%mapcar (:lambda (elt) (prelude:%compile elt env)) arg-list))))

(mu:intern prelude "%compile-quoted-lambda-arg-list"
   (:lambda (function arg-list env)
     (prelude:%compile-lambda-arg-list function (prelude:%mapcar (:lambda (elt) (prelude:%compile elt env)) arg-list))))

;;;
;;; compile function applications to mu forms
;;;
(mu:intern prelude "%compile-lambda-call"
   (:lambda (lambda-form arg-list env)
     ((:lambda (compiled-function)
        (:if (prelude:functionp compiled-function)
             (:if (prelude:%prelude-function-p compiled-function)
                  `(,prelude:%fn-apply-quoted ,compiled-function ,(prelude:%compile-lambda-arg-list compiled-function arg-list env))
                  `(mu:apply ,compiled-function ,(prelude:%compile-arg-list arg-list env)))
             (prelude:raise compiled-function 'prelude:%compile-lambda-call "illegal function call")))
     (prelude:%compile lambda-form env))))

(mu:intern prelude "%compile-symbol-call"
   (:lambda (function-symbol arg-list env)
      (:if (prelude:boundp function-symbol)
           ((:lambda (function)
               (:if (prelude:functionp function)
                    (:if (prelude:%prelude-function-p function)
                         `(,prelude:%fn-apply-quoted ,function ,(prelude:%compile-lambda-arg-list function arg-list env))
                         `(mu:apply ,function ,(prelude:%compile-arg-list arg-list env)))
                    (prelude:raise function 'prelude:%compile-symbol-call "illegal function call")))
            (prelude:symbol-value function-symbol))
           `(,prelude:apply ,function-symbol ,(prelude:%compile-arg-list arg-list env)))))

(mu:intern prelude "%compile-funcall"
   (:lambda (function-form arg-list env)
      (:if (prelude:consp function-form)
           (prelude:%compile-lambda-call function-form arg-list env)
           (:if (mu:eq :symbol (mu:type-of function-form))
                (prelude:%compile-symbol-call function-form arg-list env)
                (prelude:raise function-form 'prelude:%compile-funcall "illegal function call")))))

;;;
;;; apply closure to argument list
;;; sic transit
(mu:intern prelude "%describe-fn"
   (:lambda (fn)
     (:if (prelude:%prelude-function-p fn)
          ((:lambda (lambda arity fn env)
             (prelude:eprint "" "%describe-fn:prelude")
             (prelude:eprint lambda "    lambda")
             (prelude:eprint arity "    arity")
             (prelude:eprint fn "    fn")
             (prelude:eprint env "    env"))
           (prelude:%fn-prop :lambda fn)
           (prelude:%fn-prop :arity fn)
           (prelude:%fn-prop :fn fn)
           (prelude:%fn-prop :env fn))
          (prelude:eprint (mu:view fn) "%describe-fn:mu-fn"))))

(mu:intern prelude "%fn-apply"
   (:lambda (fn arg-list)
     ;;; (prelude:%describe-fn fn)
     ;;; (prelude:eprint arg-list "%fn-apply-quoted:arg list")
     ((:lambda (env)
        (prelude:%mapc mu:frame-push env)
         ((:lambda (value)
            (prelude:%mapc (:lambda (frame) (mu:frame-pop (mu:car frame))) env)
            value)
          ((:lambda (lib-fn)
             (mu:apply lib-fn (mu:eval (prelude:%lambda-arg-list fn arg-list))))
           (prelude:%fn-prop :fn fn))))
       (prelude:%fn-prop :env fn))))

(mu:intern prelude "%fn-apply-quoted"
   (:lambda (fn arg-list)
     ;;; (prelude:%describe-fn fn)
     ;;; (prelude:eprint arg-list "%fn-apply-quoted:arg list")
      ((:lambda (env)
        (prelude:%mapc mu:frame-push env)
        ((:lambda (value)
           (prelude:%mapc (:lambda (frame) (mu:frame-pop (mu:car frame))) env)
           value)
         ((:lambda (lib-fn)
             (mu:apply lib-fn arg-list))
          (prelude:%fn-prop :fn fn))))
        (prelude:%fn-prop :env fn))))
