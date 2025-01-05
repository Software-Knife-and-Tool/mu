;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; defun/defmacro
;;;
(mu:eval (core:compile
  '(%defmacro defun (name lambda &rest body)
     `(mu:intern mu:%null-ns% ,(mu:symbol-name name) (%lambda ,lambda ,@body)))))

(mu:eval (core:compile
  '(%defmacro defmacro (name lambda &rest body)
     `(%defmacro ,(mu:symbol-name name) ,lambda ,@body))))
