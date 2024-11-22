;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude namespace
;;;
(mu:intern mu:%null-ns% "prelude" (mu:make-namespace "prelude"))
(mu:intern prelude "+version+" "0.0.1")

(core:provide
 "prelude"
 '("list.l"
   "loader.l"))

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