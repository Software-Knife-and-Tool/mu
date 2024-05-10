;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  environment
;;;
(core:intern prelude "environmentp"
   (:lambda (env)
      (:if (prelude:%prelude-type-p env)
           (core:eq 'env (prelude:type-of env))
           ())))

(core:intern prelude "make-environment"
   (:lambda (bindings)
     (:if (prelude:listp bindings)
          ((:lambda (env)
             (prelude:warn env "make-environment: env")
             (prelude:mapc
              (:lambda (pair)
                ((:lambda (symbol value)
                   (:if (prelude:symbolp symbol)
                        (prelude:%add-env env symbol value)
                        (prelude:raise symbol 'prelude:make-environment "not a symbol")))
                 (core:car pair)
                 (core:cdr pair)))
              bindings)
             env)
           (prelude:%make-env))
          (prelude:raise symbol 'prelude:make-environment "not a symbol"))))

;;;
;;; implementation
;;;
(core:intern prelude "%get-env"
   (:lambda (env symbol)
     (:if (prelude:symbolp symbol)
         (:if (prelude:environmentp env)
              ((:lambda (ns)
                 ((:lambda (sym)
                     (:if (prelude:null sym)
                          (prelude:raise symbol 'prelude:%get-env "symbol not in map")
                          sym))
                  (core:find ns (core:symbol-name symbol))))
                 (prelude:%env-prop :symbols env))
              (prelude:raise env 'prelude:%get-env "not an env"))
         (prelude:raise symbol 'prelude:%get-env "not a symbol"))))

(core:intern prelude "%add-env"
   (:lambda (env symbol value)
     (:if (prelude:symbolp symbol)
         (:if (prelude:environmentp env)
              ((:lambda (ns)
                 ((:lambda (sym)
                     (:if (prelude:null sym)
                          (prelude:raise symbol 'prelude:%add-env "symbol already in map")
                          (core:intern ns (core:symbol-name symbol) value)))
                  (core:find ns (core:symbol-name symbol))))
               (prelude:%env-prop :symbols env))
              (prelude:raise env 'prelude:%add-env "not an env"))
         (prelude:raise symbol 'prelude:%add-env "not a symbol"))))

(core:intern prelude "%make-env"
   (:lambda ()
     (prelude:%make-type "env" `(,(core:cons :symbols (core:make-ns))))))

(core:intern prelude "%env-prop"
   (:lambda (prop env)
      (:if (prelude:findl (:lambda (env-prop) (core:eq prop env-prop)) '(:symbols))
           (core:cdr (prelude:%type-ref prop env))
           (prelude:raise prop 'prelude:%env-prop "not an environment property"))))
