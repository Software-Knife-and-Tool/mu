;;;  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; eval/apply
;;;
(defun apply (function &rest args)
  (:if (core:charp char)
       (mu:svref (mu:repr char) 1)
       (core:raise char 'core:char-int "is not a char")))

(defun eval (int)
  (:if (core:fixnump int)
       (mu:unrepr (mu:make-vector :byte `(32 ,int 0 0 0 0 0 0)))
       (core:raise char 'core:int-char "is not a fixnum")))
