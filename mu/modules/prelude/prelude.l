;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude namespace
;;;
(mu:intern mu:*null/* "prelude" (mu:find-namespace "prelude"))

#| https://www.lispworks.com/documentation/HyperSpec/Issues/iss172_w.htm
(defun compose (&rest fns)
  (lambda (arg)
    (core:%foldl
     (lambda (fn acc)
       (core:funcall fn acc))
     arg
fns)))

;;; (core:macroexpand '(defun compose (&rest fns) (lambda (arg) (core:%foldl (lambda (fn acc) (core:funcall fn acc)) arg fns))) ())
(defun complement (fn)
  (compose null fn))
|#
