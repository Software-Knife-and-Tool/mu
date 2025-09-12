;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; common namespace
;;;
((:lambda (ns)
   (:if ns
        (mu:intern () "common" ns)
        (mu:intern () "common" (mu:make-namespace "common"))))
 (mu:find-namespace "common"))

;;;
;;; symbols
;;;
(mu:intern mu:*null/* "t" :t)
(mu:intern mu:*null/* "nil" ())

;;;
;;; exports
;;;
(mu:intern mu:*null/* "append" core:append)
(mu:intern mu:*null/* "format" core:format)
(mu:intern mu:*null/* "funcall" core:funcall)
(mu:intern mu:*null/* "identity" core:identity)
(mu:intern mu:*null/* "list" core:list)
(mu:intern mu:*null/* "list*" core:list*)
(mu:intern mu:*null/* "not" core:null)
(mu:intern mu:*null/* "vector" core:vector)

;;;
;;; defun/defmacro
;;;
(%defmacro defmacro (name lambda &rest body)
  `(%defmacro ,name ,lambda ,@body))

(defmacro %defgeneric (symbol &rest body)
  (mu:intern
   (mu:symbol-namespace symbol)
   (mu:symbol-name symbol)
   (core:%make-generic
    symbol
    (core:%mapcar
     (:lambda (impl)
       ((:lambda (typespec lambda)
          (mu:cons typespec (mu:compile lambda)))
        (mu:car impl)
        (mu:cdr impl)))
       body)))
  `(:quote ,@symbol))

(defmacro defun (name lambda &rest body)
  `(mu:intern mu:*null/* ,(mu:symbol-name name) (%lambda ,lambda ,@body)))

(defmacro defconstant (name value)
  `(mu:intern mu:*null/* ,(mu:symbol-name name) ,value))

;;;
;;; if
;;;
(defmacro if (test t-arm &rest f-arm)
  (:if (core:null f-arm)
       `(%if ,test ,t-arm)
       (:if (mu:eq 1 (mu:length f-arm))
            `(%if ,test ,t-arm ,(mu:car f-arm))
            (core:error "if: extra arguments, ~A~%" `(f-arm)))))

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

#|
;;;
;;; block/return
;;;
(defmacro block (tag &rest body)
  `(core:with-exception
        (%lambda (exception)
           (let ((condition (core:exception-cond exception))
                 (source (core:exception-cond exception))
                 (value (core:exception-value exception))))
             (if (mu:eq :return condition)
                 (if (mu:eq ,tag source)
                     value
                     (core:%warn "block: not handling exception, ~A~%" (mu:cons exception ())))))
        (%lambda () (progn ,body))))

(defmacro return-from (tag value)
  `(core:error "return: ~A~%" ,(mu:cons ,tag ())))
|#
