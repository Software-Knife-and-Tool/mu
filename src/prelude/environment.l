;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  environment
;;;
(lib:intern prelude "environmentp"
   (:lambda (env)
      (:if (prelude:%prelude-type-p env)
           (lib:eq 'env (prelude:type-of env))
           ())))

(lib:intern prelude "make-environment"
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
                 (lib:car pair)
                 (lib:cdr pair)))
              bindings)
             env)
           (prelude:%make-env))
          (prelude:raise symbol 'prelude:make-environment "not a symbol"))))

;;;
;;; implementation
;;;
(lib:intern prelude "%get-env"
   (:lambda (env symbol)
     (:if (prelude:symbolp symbol)
         (:if (prelude:environmentp env)
              ((:lambda (ns)
                 ((:lambda (sym)
                     (:if (prelude:null sym)
                          (prelude:raise symbol 'prelude:%get-env "symbol not in map")
                          sym))
                  (lib:find ns (lib:symbol-name symbol))))
                 (prelude:%env-prop :symbols env))
              (prelude:raise env 'prelude:%get-env "not an env"))
         (prelude:raise symbol 'prelude:%get-env "not a symbol"))))

(lib:intern prelude "%add-env"
   (:lambda (env symbol value)
     (:if (prelude:symbolp symbol)
         (:if (prelude:environmentp env)
              ((:lambda (ns)
                 ((:lambda (sym)
                     (:if (prelude:null sym)
                          (prelude:raise symbol 'prelude:%add-env "symbol already in map")
                          (lib:intern ns (lib:symbol-name symbol) value)))
                  (lib:find ns (lib:symbol-name symbol))))
               (prelude:%env-prop :symbols env))
              (prelude:raise env 'prelude:%add-env "not an env"))
         (prelude:raise symbol 'prelude:%add-env "not a symbol"))))

(lib:intern prelude "%make-env"
   (:lambda ()
     (prelude:%make-type "env" `(,(lib:cons :symbols (lib:make-ns))))))

(lib:intern prelude "%env-prop"
   (:lambda (prop env)
      (:if (prelude:findl (:lambda (env-prop) (lib:eq prop env-prop)) '(:symbols))
           (lib:cdr (prelude:%type-ref prop env))
           (prelude:raise prop 'prelude:%env-prop "not an environment property"))))
