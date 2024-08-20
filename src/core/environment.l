;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  environment
;;;
(mu:intern core "environmentp"
   (:lambda (env)
      (:if (core:%core-type-p env)
           (mu:eq 'env (core:type-of env))
           ())))

(mu:intern core "%make-env"
   (:lambda ()
     (core:%make-type "env" `(,(mu:cons :symbols (mu:make-namespace))))))

(mu:intern core "%env-prop"
   (:lambda (prop env)
      (:if (core:findl (:lambda (env-prop) (mu:eq prop env-prop)) '(:symbols))
           (mu:cdr (core:%type-ref prop env))
           (core:raise prop 'core:%env-prop "not an environment property"))))

;;;
;;; constructor
;;;
(mu:intern core "make-environment"
   (:lambda (bindings)
     (:if (core:listp bindings)
          ((:lambda (env)
             (core:mapc
              (:lambda (pair)
                ((:lambda (symbol value)
                   (:if (core:symbolp symbol)
                        (core:%add-env env symbol value)
                        (core:raise symbol 'core:make-environment "not a symbol")))
                 (mu:car pair)
                 (mu:cdr pair)))
              bindings)
             env)
           (core:%make-env))
          (core:raise symbol 'core:make-environment "not a symbol"))))

;;;
;;; functions
;;;
(mu:intern core "%get-env"
   (:lambda (env symbol)
     (:if (core:symbolp symbol)
         (:if (core:environmentp env)
              ((:lambda (ns)
                 ((:lambda (sym)
                     (:if (core:null sym)
                          (core:raise symbol 'core:%get-env "symbol not in map")
                          sym))
                  (mu:find ns (mu:symbol-name symbol))))
                 (core:%env-prop :symbols env))
              (core:raise env 'core:%get-env "not an env"))
         (core:raise symbol 'core:%get-env "not a symbol"))))

(mu:intern core "%add-env"
   (:lambda (env symbol value)
     (:if (core:symbolp symbol)
         (:if (core:environmentp env)
              ((:lambda (ns)
                 ((:lambda (sym)
                     (:if (core:null sym)
                          (core:raise symbol 'core:%add-env "symbol already in map")
                          (mu:intern ns (mu:symbol-name symbol) value)))
                  (mu:find ns (mu:symbol-name symbol))))
               (core:%env-prop :symbols env))
              (core:raise env 'core:%add-env "not an env"))
         (core:raise symbol 'core:%add-env "not a symbol"))))
