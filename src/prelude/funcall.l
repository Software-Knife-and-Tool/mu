;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; compile applications
;;;

;;;
;;; argument lists
;;;
(libcore:intern :prelude "%arg-list"
    (:lambda (arg-list)
      (prelude:foldr
       (:lambda (elt acc)
         `(libcore:cons ,elt ,acc))
       ()
       arg-list)))

(libcore:intern :prelude "%lambda-arg-list"
   (:lambda (function arg-list)
      (:if (prelude:%prelude-function-p function)
           ((:lambda (rest nreqs)
               (:if (prelude:%andf rest (prelude:zerop nreqs))
                    `(libcore:cons ,(prelude:%arg-list arg-list) ())
                    ((:lambda (reqs rest)
                        (prelude:%arg-list `(,@reqs ,(prelude:%arg-list rest))))
                     (prelude:dropr arg-list (libcore:fx-sub (libcore:length arg-list) nreqs))
                     (prelude:dropl arg-list nreqs))))
            (prelude:%funct-prop :rest function)
            (prelude:%funct-prop :arity function))
           (prelude:%arg-list arg-list))))

(libcore:intern :prelude "%quoted-lambda-arg-list"
  (:lambda (fn args)
    (:if (prelude:%prelude-function-p fn)
         (:if (prelude:%funct-prop :rest fn)
              (:if (prelude:%andf (prelude:%funct-prop :rest fn) (prelude:zerop (prelude:%funct-prop :arity fn)))
                   `(,args)
                   ((:lambda (reqs rest)
                      (prelude:append
                       reqs
                       `(,rest)))
                    (prelude:dropr args (libcore:fx-sub (libcore:length args) (prelude:%funct-prop :arity fn)))
                    (prelude:dropl args (prelude:%funct-prop :arity fn))))
              args)
         args)))

;;;
;;; compiled argument lists
;;;
(libcore:intern :prelude "%compile-arg-list"
   (:lambda (arg-list env)
      (prelude:foldr
       (:lambda (elt acc)
          `(libcore:cons ,@(libcore:cons elt `(,acc))))
       ()
       (prelude:mapcar
        (:lambda (form)
           (prelude:%compile form env))
        arg-list))))

(libcore:intern :prelude "%compile-lambda-arg-list"
   (:lambda (function arg-list env)
     (prelude:%lambda-arg-list function (prelude:%mapcar (:lambda (elt) (prelude:%compile elt env)) arg-list))))

(libcore:intern :prelude "%compile-quoted-lambda-arg-list"
   (:lambda (function arg-list env)
     (prelude:%compile-lambda-arg-list function (prelude:%mapcar (:lambda (elt) (prelude:%compile elt env)) arg-list))))

;;;
;;; compile-funcall
;;;
;;; compile function applications to mu-appliable forms
;;;
(libcore:intern :prelude "%compile-lambda-call"
   (:lambda (lambda-form arg-list env)
      ((:lambda (compiled-function)
        (:if (prelude:functionp compiled-function)
             (:if (prelude:%prelude-function-p compiled-function)
                  `(prelude:%funct-apply-quoted ,compiled-function ,(prelude:%compile-lambda-arg-list compiled-function arg-list env))
                  `(libcore:apply ,compiled-function ,(prelude:%compile-arg-list arg-list env)))
             (prelude:raise compiled-function 'prelude:%compile-lambda-call "illegal function call")))
     (prelude:%compile lambda-form env))))

(libcore:intern :prelude "%compile-symbol-call"
   (:lambda (function-symbol arg-list env)
      (:if (prelude:boundp function-symbol)
           ((:lambda (function)
               (:if (prelude:functionp function)
                    (:if (prelude:%prelude-function-p function)
                         `(prelude:%funct-apply-quoted ,function ,(prelude:%compile-lambda-arg-list function arg-list env))
                         `(libcore:apply ,function ,(prelude:%compile-arg-list arg-list env)))
                    (prelude:raise function 'prelude:%compile-symbol-call "illegal function call")))
            (prelude:symbol-value function-symbol))
           `(prelude:apply ,function-symbol ,(prelude:%compile-arg-list arg-list env)))))

(libcore:intern :prelude "%compile-funcall"
   (:lambda (function-form arg-list env)
      (:if (prelude:consp function-form)
           (prelude:%compile-lambda-call function-form arg-list env)
           (:if (libcore:eq :symbol (libcore:type-of function-form))
                (prelude:%compile-symbol-call function-form arg-list env)
                (prelude:raise function-form 'prelude:%compile-funcall "illegal function call")))))

;;;
;;; apply closure to argument list
;;;
(libcore:intern :prelude "%funct-apply"
   (:lambda (funct arg-list)
      ((:lambda (env)
        (prelude:%mapc libcore:fr-push env)
        ((:lambda (value)
           (prelude:%mapc (:lambda (frame) (libcore:fr-pop (libcore:car frame))) env)
           value)
         ((:lambda (mu-fn)
             (libcore:apply mu-fn (libcore:eval (prelude:%lambda-arg-list funct arg-list))))
          (prelude:%funct-prop :fn funct))))
        (prelude:%funct-prop :env funct))))

(libcore:intern :prelude "%funct-apply-quoted"
   (:lambda (funct arg-list)
      ((:lambda (env)
        (prelude:%mapc libcore:fr-push env)
        ((:lambda (value)
           (prelude:%mapc (:lambda (frame) (libcore:fr-pop (libcore:car frame))) env)
           value)
         ((:lambda (mu-fn)
             (libcore:apply mu-fn arg-list))
          (prelude:%funct-prop :fn funct))))
        (prelude:%funct-prop :env funct))))
