;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; apply functions
;;;
(mu:intern core "%fapply"
   (:lambda (core-function mu-function arg-list)
     ((:lambda (env)
        ;;; (core:warn `(,mu-function ,arg-list) "%fapply")
        (mu:apply mu-function arg-list))
     (core:%core-function-prop :env core-function))))

(mu:intern core "%apply"
   (:lambda (function arg-list)
     (:if (core:%core-function-p function)
          ((:lambda (arg-list)
             (core:%fapply
              function
              (mu:compile (core:%core-function-prop :mu function))
              arg-list))
           (mu:eval (core:%compile-lambda-arg-list function arg-list ())))
          (mu:apply function arg-list))))

(mu:intern core "apply"
   (:lambda (function arg-list)
     (:if (core:functionp function)
          (core:%apply function arg-list)
          (core:raise function 'core:apply "not a function designator"))))