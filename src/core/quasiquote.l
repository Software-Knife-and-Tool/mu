;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; quasiquote
;;;
(mu:intern (mu:find-namespace "") "%quasi%" ())

(mu:intern core "%list"
   (:lambda (form)
     `(,form)))

(mu:intern core "%quasi-func-map"
   (:lambda (key map)
     (mu:cdr (core:%assq key map))))

(mu:intern core "%quasi-read"
   (:lambda (stream)
     ((:lambda (ch syntax-map)
        (:if (core:null ch)
             (core:error stream "backquote: early end of file~%" ())
             ((:lambda (type-map)
                (:if (core:null type-map)
                     `(:form ,(core:%read-dispatch ch stream))
                     (mu:apply (mu:cdr type-map) ())))
                (core:%assq ch syntax-map))))
        (core:%read-consume-ws stream)
        `(,(mu:cons #\( (:lambda () `(:list ,(core:%quasi-read-list stream))))
           ,(mu:cons #\) (:lambda () `(:list-)))
           ,(mu:cons #\, (:lambda ()
                           ((:lambda (ch)
                             (:if (core:null ch)
                                  (core:error stream "backquote: early end of file~%" ())
                                  (:if (mu:eq #\@ ch)
                                       `(:comma@ ,(core:read stream () ()))
                                       (core:%arg2
                                          (mu:unread-char ch stream)
                                          `(:form ,(core:read stream () ()))))))
                            (mu:read-char stream () ()))))
           ,(mu:cons #\' (:lambda () `(:qquote ,(core:%quasi-read stream))))
           ,(mu:cons #\` (:lambda () `(:quasi ,(core:read stream () ()))))))))

(mu:intern core "%quasi-read-list"
   (:lambda (stream)
     ((:lambda (syntax-map)
         (mu:cdr
          (mu:fix
           (:lambda (loop)
             ((:lambda (eol list)
                (:if eol
                     loop
                     ((:lambda (syntax)
                        ((:lambda (type expr)
                           (:if (core:null syntax)
                                (core:error stream "qquote: early end of file~%" ())
                                ((:lambda (type-fn)
                                   (mu:apply type-fn `(,expr ,list)))
                                 (core:%quasi-func-map type syntax-map))))
                         (mu:car syntax)
                         (mu:cdr syntax)))
                      (core:%quasi-read stream))))
              (mu:car loop)
              (mu:cdr loop)))
           ())))
      `(,(mu:cons :comma   (:lambda (expr list)
                             (mu:cons () (mu:append list (core:%list `(:l-comma ,(mu:car expr)))))))
         ,(mu:cons :comma@ (:lambda (expr list)
                             (mu:cons () (mu:append list (core:%list `(:comma@ ,(mu:car expr)))))))
         ,(mu:cons :form   (:lambda (expr list)
                             (mu:cons () (mu:append list (core:%list `(:l-form ,(mu:car expr)))))))
         ,(mu:cons :list   (:lambda (expr list)
                             (:if (core:null (mu:car expr))
                                  (mu:cons () (mu:append list (core:%list `(:l-form ()))))
                                  (:if (core:%consp (mu:car expr))
                                       ((:lambda (form)
                                          (mu:cons () (mu:append list (core:%list `(:l-form ,form)))))
                                        (core:%mapcar (:lambda (expr) (mu:nth 1 expr)) (mu:car expr)))
                                       (mu:cons () (mu:append list (core:%list `(:l-list ,(mu:car expr)))))))))
         ,(mu:cons :list-  (:lambda (expr list)
                             (mu:cons :t list)))
         ,(mu:cons :quasi  (:lambda (expr list)
                             (mu:cons () (mu:append list `(,(core:%quasi-read stream))))))))))

(mu:intern core "%quasi-compile"
   (:lambda (quasi-expr)
     ((:lambda (type expr compiler-map)
        ((:lambda (type-fn)
           (:if (core:null type-fn)
                (core:error type "backquote: unmapped type ~A~%" `(,type))
                (mu:apply type-fn `(,expr))))
         (core:%quasi-func-map type compiler-map)))
      (mu:nth 0 quasi-expr)
      (mu:nth 1 quasi-expr)
      `(,(mu:cons :l-comma  (:lambda (expr) `(mu:cons ,expr ())))
         ,(mu:cons :l-list  (:lambda (expr)
                              ((:lambda (loop)
                                 (mu:apply loop `(,loop ,expr)))
                               (:lambda (loop list)
                                 (:if (core:null list)
                                      list
                                      `(mu:append
                                        ,(core:%quasi-compile (mu:car list))
                                        ,(mu:apply loop `(,loop ,(mu:cdr list)))))))))
         ,(mu:cons :l-form  (:lambda (expr)
                              ((:lambda (quote)
                               `(mu:cons ,quote ()))
                               `(:quote ,expr))))
         ,(mu:cons :comma@ (:lambda (expr) expr))
         ,(mu:cons :list    (:lambda (list)
                              ((:lambda (loop)
                                 (mu:apply loop `(,loop ,list)))
                               (:lambda (loop list)
                                 (:if (core:null list)
                                      list
                                      `(mu:append
                                        ,(core:%quasi-compile (mu:car list))
                                        ,(mu:apply loop `(,loop ,(mu:cdr list)))))))))
         ,(mu:cons :form    (:lambda (expr)
                              `(:quote ,expr)))
         ,(mu:cons :quasi   (:lambda (expr)
                              ((:lambda (type)
                                 (:if (mu:eq :form type)
                                      (mu:cdr expr)
                                      (:if (mu:eq :quote type)
                                           (core:%quasi-compile expr)
                                           ())))
                               (mu:car expr))))
         ,(mu:cons :qquote   (:lambda (expr)
                               `(:quote ,(core:%quasi-compile expr))))))))

(mu:intern core "%read-quasi"
   (:lambda (char stream)
     `(%quasi% ,(core:%quasi-read stream))))
