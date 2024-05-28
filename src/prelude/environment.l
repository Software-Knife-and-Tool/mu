;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  environment
;;;
(crux:intern prelude "environmentp"
   (:lambda (env)
      (:if (prelude:%prelude-type-p env)
           (crux:eq 'env (prelude:type-of env))
           ())))

(crux:intern prelude "make-environment"
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
                 (crux:car pair)
                 (crux:cdr pair)))
              bindings)
             env)
           (prelude:%make-env))
          (prelude:raise symbol 'prelude:make-environment "not a symbol"))))

;;;
;;; implementation
;;;
(crux:intern prelude "%get-env"
   (:lambda (env symbol)
     (:if (prelude:symbolp symbol)
         (:if (prelude:environmentp env)
              ((:lambda (ns)
                 ((:lambda (sym)
                     (:if (prelude:null sym)
                          (prelude:raise symbol 'prelude:%get-env "symbol not in map")
                          sym))
                  (crux:find ns (crux:symbol-name symbol))))
                 (prelude:%env-prop :symbols env))
              (prelude:raise env 'prelude:%get-env "not an env"))
         (prelude:raise symbol 'prelude:%get-env "not a symbol"))))

(crux:intern prelude "%add-env"
   (:lambda (env symbol value)
     (:if (prelude:symbolp symbol)
         (:if (prelude:environmentp env)
              ((:lambda (ns)
                 ((:lambda (sym)
                     (:if (prelude:null sym)
                          (prelude:raise symbol 'prelude:%add-env "symbol already in map")
                          (crux:intern ns (crux:symbol-name symbol) value)))
                  (crux:find ns (crux:symbol-name symbol))))
               (prelude:%env-prop :symbols env))
              (prelude:raise env 'prelude:%add-env "not an env"))
         (prelude:raise symbol 'prelude:%add-env "not a symbol"))))

(crux:intern prelude "%make-env"
   (:lambda ()
     (prelude:%make-type "env" `(,(crux:cons :symbols (crux:make-ns))))))

(crux:intern prelude "%env-prop"
   (:lambda (prop env)
      (:if (prelude:findl (:lambda (env-prop) (crux:eq prop env-prop)) '(:symbols))
           (crux:cdr (prelude:%type-ref prop env))
           (prelude:raise prop 'prelude:%env-prop "not an environment property"))))
