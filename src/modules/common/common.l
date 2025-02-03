;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; common namespace
;;;
(mu:intern mu:%null-ns% "common" (mu:find-namespace "common"))
(mu:intern mu:%null-ns% "t" :t)
(mu:intern mu:%null-ns% "nil" ())

;;;
;;; exports
;;;
(mu:intern mu:%null-ns% "append" core:append)
(mu:intern mu:%null-ns% "list" core:list)
(mu:intern mu:%null-ns% "list*" core:list*)
(mu:intern mu:%null-ns% "vector" core:vector)
(mu:intern mu:%null-ns% "format" core:format)
(mu:intern mu:%null-ns% "funcall" core:funcall)
(mu:intern mu:%null-ns% "not" core:null)

;;;
;;; defun/defmacro
;;;
(%defmacro defmacro (name lambda &rest body)
  `(%defmacro ,name ,lambda ,@body))

(defmacro defun (name lambda &rest body)
  `(mu:intern mu:%null-ns% ,(mu:symbol-name name) (%lambda ,lambda ,@body)))

(defmacro defconstant (name value)
  `(mu:intern mu:%null-ns% ,(mu:symbol-name name) ,value))

;;;
;;; progs
;;;
(defmacro prog1 (form-1 &rest forms)
  (let ((g (core:gensym)))
     `(let ((,g ,form-1))
        ,@forms
        ,g)))

(defmacro prog2 (form-1 form-2 &rest forms)
  (let ((g (core:gensym)))
     `(let ((,g (progn ,form-1 ,form-2)))
        ,@forms
        ,g)))
