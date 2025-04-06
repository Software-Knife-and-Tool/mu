;;;  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; eval/apply
;;;
(defun apply (function &rest args)
  (cond
    ((core:functionp function) (core:apply function args))
    ((common:genericp function) (common:%apply-generic function args))
    (:t (core:error function "apply: not a function~%" ()))))

(defun eval (int)
  (:if (core:fixnump int)
       (mu:unrepr (mu:make-vector :byte `(32 ,int 0 0 0 0 0 0)))
       (core:raise char 'core:int-char "is not a fixnum")))
