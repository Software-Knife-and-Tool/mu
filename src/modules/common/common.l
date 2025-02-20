;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; common namespace
;;;
(mu:intern mu:*mu/null* "common" (mu:find-namespace "common"))
(mu:intern mu:*mu/null* "t" :t)
(mu:intern mu:*mu/null* "nil" ())

;;;
;;; exports
;;;
(mu:intern mu:*mu/null* "append" core:append)
(mu:intern mu:*mu/null* "format" core:format)
(mu:intern mu:*mu/null* "funcall" core:funcall)
(mu:intern mu:*mu/null* "identity" core:identity)
(mu:intern mu:*mu/null* "list" core:list)
(mu:intern mu:*mu/null* "list*" core:list*)
(mu:intern mu:*mu/null* "not" core:null)
(mu:intern mu:*mu/null* "vector" core:vector)

;;;
;;; char functions
;;;
(mu:intern mu:*mu/null* "char-int"
   (:lambda (char)
     (:if (core:charp char)
          (mu:svref (mu:repr :vector char) 1)
          (core:raise char 'core:char-int "is not a char"))))

(mu:intern mu:*mu/null* "int-char"
   (:lambda (int)
     (:if (core:fixnump int)
          (mu:repr :t (mu:make-vector :byte `(32 ,int 0 0 0 0 0 0)))
          (core:raise char 'core:int-char "is not a fixnum"))))

;;;
;;; defun/defmacro
;;;
(%defmacro defmacro (name lambda &rest body)
  `(%defmacro ,name ,lambda ,@body))

(defmacro defun (name lambda &rest body)
  `(mu:intern mu:*mu/null* ,(mu:symbol-name name) (%lambda ,lambda ,@body)))

(defmacro defconstant (name value)
  `(mu:intern mu:*mu/null* ,(mu:symbol-name name) ,value))

;;;
;;; if
;;;
(defmacro if (test t-arm &rest f-arm)
  (:if (core:null f-arm)
       `(%if ,test ,t-arm)
       (:if (mu:eq 1 (mu:length f-arm))
            `(%if ,test ,t-arm ,(mu:car f-arm))
            (core:raise f-arm 'if "extra arguments"))))

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

;;;
;;; block/return
;;;
(defmacro block (tag &rest body)
  `(core:with-exception
        (%lambda (exception)
          (let ((condition (core:%exception-prop :return exception)))
            (if (mu:eq ,tag condition)
                (core:%exception-prop :value exception)
                (core:warn exception 'block "not handling exception"))))
        (%lambda () (progn ,body))))

(defmacro return-from (tag value)
  `(core:raise (:quote tag) 'return ""))
