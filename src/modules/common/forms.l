;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; common namespace
;;;
#|
(mu:intern common "identity"
   (:lambda (obj)
     obj))

(defun vector (&rest elements)
  (list-sv :t elements))

(defmacro bind-seq (pattern source &rest body)
  (errorp-unless consp pattern "bind-seq: pattern not a list")
  (let* ((source-gsym (gensym))
         (destruct
          (lambda ()
            (let ((nths
                   (foldr
                    (lambda (sym acc)
                      (errorp-unless symbolp sym "bind-seq: not a symbol")
                      (let* ((nth (car acc))
                             (phrases (cdr acc))
                             (phrase (list sym (core:list 'elt source-gsym nth))))
                        (cons (1- nth) (cons phrase phrases))))
                    (cons (1- (length pattern)) ())
                    pattern)))
              (cdr nths)))))
    (list* 'let* (cons (core:list source-gsym source) (destruct)) body)))

;;;
;;; do macros
;;;
(defmacro do* (bind-form end-form &rest body)
  (errorp-unless consp end-form "do: binding form not a list")
  (mapc
   (lambda (phrase)
     (cond
       ((symbolp phrase)
        (errorp-unless uninternedp phrase "do: binding symbol is interned"))
       ((consp phrase)
        (errorp-unless (lambda (phrase) (fixnum< (length phrase) 4)) phrase "do: binding phrase format length")
        (errorp-unless uninternedp (nth 0 phrase) "do: binding phrase format type "))
       (t (error phrase "do: binding phrase type"))))
   bind-form)
  (errorp-unless consp end-form "do: end form not a list")
  (let ((self-gensym (gensym))
        (exit-gensym (gensym))
        (return-gensym (gensym))
        (var-list (mapcar (lambda (phrase) (%if (symbolp phrase) phrase (nth 0 phrase))) bind-form))
        (init-list (mapcar (lambda (phrase) (%if (symbolp phrase) () (nth 1 phrase))) bind-form))
        (step-list (mapcar (lambda (phrase) (%if (symbolp phrase) () (nth 2 phrase))) bind-form)))
    (list 'fix*
          (list 'lambda (list* self-gensym exit-gensym return-gensym var-list)
                (list 'if (nth 0 end-form)
                      (list 'progn
                            (list '%env%:fp-setv (list core::fn-frame-id self-gensym) 1 0)
                            (list '%env%:fp-setv (list core::fn-frame-id self-gensym) 2 (nth 1 end-form)))
                      (list 'progn
                            (cons 'progn body)
                            (list '%env%:fp-setv (list core::fn-frame-id self-gensym) 1 (list 'null exit-gensym))
                            (cons 'progn
                                  (maplist
                                   (lambda (step-cdr)
                                     (list '%env%:fp-setv (list core::fn-frame-id self-gensym) (fixnum+ 2 (length step-cdr)) (car step-cdr)))
                                   step-list)))))
          (list 'list* () () (cons 'list init-list)))))

;;; (do let-form (end-test result) . body)

(defun core::reclose (closure)
  (errorp-unless functionp closure "reclose: not a function")
  (let ((closed (core::lambda-closure (core::fn-lambda-desc closure))))
    (errorp-unless (lambda (list) (fixnum< 0 (length list))) closed "reclose environment botch")
    (%env%:fnv-set (car closed) 0 closure))
  closure)

(defmacro recur (name lambda &rest body)
  (let ((gsym (gensym)))
    (list 'let
          (list (list gsym
                      (list 'let
                            (list (list name (list 'lambda lambda)))
                            (list* 'lambda lambda body))))
          (list 'core::reclose gsym))))

(defmacro letf (binds &rest body)
  (errorp-unless listp binds "letf: not a list")
  (list* 'let
         (mapcar
           (lambda (bind)
             (list (car bind)
                   (core::funcall recur bind)))
             binds)
         body))

(defmacro letf* (binds &rest body)
  (errorp-unless listp binds "letf*: not a list")
  ((lambda (fn)
     (%if binds
       (fn fn binds)
       (list* 'progn body)))
   (lambda (fn binds)
     (%if (cdr binds)
         (list 'letf (list (car binds)) (core::funcall fn (list fn (cdr binds))))
         (list* 'letf (list (car binds)) body)))))

(defmacro flet (binds &rest body)
  (errorp-unless listp binds "flet: not a list")
  (list* 'let
         (mapcar
          (lambda (bind)
            (list (nth 0 bind)
                  (list* 'lambda (nth 1 bind) (nthcdr 2 bind))))
          binds)
         body))

(defmacro flet* (binds &rest body)
  (errorp-unless listp binds "flet*: not a list")
  ((lambda (fn)
     (%if binds
       (fn fn binds)
       (list* 'progn body)))
   (lambda (fn binds)
     (%if (cdr binds)
         (list 'flet (list (car binds)) (core::funcall fn (list fn (cdr binds))))
         (list* 'flet (list (car binds)) body)))))

(defmacro defun-rec (name lambda &rest body)
  (errorp-unless symbolp name "defun: not a symbol")
  (errorp-when keywordp name "defun: not a symbol")
  (errorp-unless listp lambda "defun: not a lambda list")
  (format t "expands to: ~A~%" (list 'lambda lambda (list* 'letf (list (list* name lambda body)) body)))
  (core::def-symbol
        name
        (core:compile (list 'lambda lambda (list* 'letf (list (list* name lambda body)) body)))
        ())
  (list :quote name))

(defun compose (&rest fns)
  (lambda (arg)
    (foldl
     (lambda (fn acc)
       (funcall fn acc))
     arg
     fns)))

(defun complement (fn)
  (compose null fn))

;;;
;;; eql
;;;
(defun eql (x y)
   (cond
     ((eq x y) t)
     ((null (eq (type-of x) (type-of y))) nil)
     ((stringp x) (string= x y))
     ((and (vectorp x) (vectorp y))
        (and (eq (vector-type x) (vector-type y))
             (eq (vector-length x) (vector-length y))
             (core::funcall eql (vector-to-list x) (vector-to-list y))))
     ((consp x)
        (and (eq (length x) (length y))
             (core::funcall eql (car x) (car y))
             (core::funcall eql (cdr x) (cdr y))))
     (t ())))

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
                (list 'if (list 'fixnum< limit-gsym 0)
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
