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
;;; do macros
;;;
;;;
;;; dotimes macro
;;;
(defmacro dotimes (ctr-list &rest body)
  (errorp-unless consp ctr-list "dotimes: not an init list")
  (error-if (fixnum< (length ctr-list) 2) ctr-list "dotimes: malformed init list")
  (error-if (fixnum< 3 (length ctr-list)) ctr-list "dotimes: malformed init list")
  (let ((count-var  (nth 0 ctr-list))
        (count-form (nth 1 ctr-list))
        (result-form (nth 2 ctr-list))
        (limit-gsym (gensym)))
    (errorp-unless symbolp count-var "dotimes: not a symbol")
    (list (list 'lambda (list limit-gsym)
                list 'if (list 'fixnum< limit-gsym 0)
                      ()
                      (list 'core::fix
                            (list 'lambda (list count-var)
                                  (list 'if (list 'eq count-var limit-gsym)
                                        count-var
                                        (list* 'progn (append body (list (list '1+ count-var))))))
                            0))
                result-form)
          count-form)))

;;;
;;; dolist macro
;;;
(defmacro dolist (init-list &rest body)
  (errorp-unless consp init-list "dolist; not an init list")
  (let ((init (nth 1 init-list))
        (sym (car init-list)))
    (errorp-unless symbolp sym "dolist; not a symbol")  
    (errorp-unless listp init "dolist; not a list")
    (list
      (list 'lambda ()
        (list
         'mapc
         (list* 'lambda (list sym) body)
         (list 'eval init))
        ()))))

(defmacro do* (bind-form end-form &rest body)
  (unless (core:consp end-form) (core:raise end-form 'do* "binding form not a list"))
  (core:mapc
   (%lambda (phrase)
      (cond
        ((core:symbolp phrase)
         (unless (core:uninternedp phrase) (core:raise phrase 'do* "binding symbol is interned")))
        ((core:consp phrase)
         (unless (lambda (phrase) (fixnum< (length phrase) 4)) phrase "do: binding phrase format length")
         (errorp-unless uninternedp (nth 0 phrase) "do: binding phrase format type "))
        (t (error phrase "do: binding phrase type"))))
   bind-form)
  (errorp-unless consp end-form "do: end form not a list")
  (let ((self-gensym (gensym))
        (exit-gensym (gensym))
        (return-gensym (gensym))
        (var-list (mapcar (lambda (phrase) (if (symbolp phrase) phrase (nth 0 phrase))) bind-form))
        (init-list (mapcar (lambda (phrase) (if (symbolp phrase) () (nth 1 phrase))) bind-form))
        (step-list (mapcar (lambda (phrase) (if (symbolp phrase) () (nth 2 phrase))) bind-form)))
    (list 'fix*
          (list 'lambda (list* self-gensym exit-gensym return-gensym var-list)
                (list 'if (nth 0 end-form)
                      (list 'progn
                            (list 'env:fp-setv (list core::fn-frame-id self-gensym) 1 0)
                            (list 'env:fp-setv (list core::fn-frame-id self-gensym) 2 (nth 1 end-form)))
                      (list 'progn
                            (cons 'progn body)
                            (list 'env:fp-setv (list core::fn-frame-id self-gensym) 1 (list 'null exit-gensym))
                            (cons 'progn
                                  (maplist
                                   (lambda (step-cdr)
                                     (list 'env:fp-setv (list core::fn-frame-id self-gensym) (fixnum+ 2 (length step-cdr)) (car step-cdr)))
                                   step-list)))))
          (list 'list* () () (cons 'list init-list)))))

(defmacro flet (binds &rest body)
  (errorp-unless listp binds "flet: not a list")
  (list* 'let
         (mapcar
          (lambda (bind)
            (list (nth 0 bind)
                  (list* 'lambda (nth 1 bind) (nthcdr 2 bind))))
          binds)
         body))
|#

#|
;;;
;;; with-open-file macro
;;;
(defmacro with-open-file (symbol path dir &rest body)
  (list 'let (list (list symbol (list core:open-file dir path)))
    (list 'unwind-protect
      (list 'lambda (list 'ex)
        (list 'print-except 'ex "with-open-file")
        (list 'close symbol))
      (list* 'lambda () (append body (list (list 'close symbol)))))))
|#
