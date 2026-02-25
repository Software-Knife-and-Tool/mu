;;;  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; apply
;;;
(defun apply (function &rest args)
  (cond
    ((core:functionp function) (core:apply function args))
    (:t (core:error "apply: not a function~%" `(,function)))))
