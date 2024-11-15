;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; apply functions
;;;
(mu:intern core "%apply"
   (:lambda (closure frame arg-list-form)
     ((:lambda (env)
        (mu:apply frame arg-list-form))
     (core:%closure-prop :env closure))))

(mu:intern core "apply"
   (:lambda (function arg-list)
     (:if (core:functionp function)
          (:if (core:%closurep function)
               (core:%apply
                function
                (core:compile (core:%closure-prop :form function))
                (mu:eval (core:%compile-lambda-arg-list function arg-list ())))
               (mu:apply function arg-list))
          (core:raise function 'core:apply "not a function designator"))))
